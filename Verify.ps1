#requires -Version 7.0
[CmdletBinding()]
param(
	[ValidateSet('ARM64', 'x64')]
	[string] $Platform = $(if ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq 'Arm64') { 'ARM64' } else { 'x64' })
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

function Assert-Example {
	param(
		[string] $Executable,
		[string[]] $Arguments = @(),
		[string] $ExpectedOutput = '',
		[int] $ExpectedExit = 0
	)

	$start = [System.Diagnostics.ProcessStartInfo]::new($Executable)
	$start.WorkingDirectory = $PSScriptRoot
	$start.UseShellExecute = $false
	$start.RedirectStandardOutput = $true
	$start.RedirectStandardError = $true
	foreach ($argument in $Arguments) {
		$start.ArgumentList.Add($argument)
	}
	$process = [System.Diagnostics.Process]::Start($start)
	try {
		$outputTask = $process.StandardOutput.ReadToEndAsync()
		$errorTask = $process.StandardError.ReadToEndAsync()
		if (!$process.WaitForExit(15000)) {
			$process.Kill($true)
			throw "Example timed out: $Executable $Arguments"
		}
		$output = $outputTask.GetAwaiter().GetResult().Replace("`r`n", "`n").TrimEnd("`n")
		$errors = $errorTask.GetAwaiter().GetResult()
		if ($process.ExitCode -ne $ExpectedExit -or $output -cne $ExpectedOutput) {
			throw "Unexpected result: $Executable $Arguments`nExit: $($process.ExitCode)`nOutput: $output`nErrors: $errors"
		}
		if ($ExpectedExit -eq 0 -and $errors.Length -ne 0) {
			throw "Unexpected stderr: $Executable $Arguments`n$errors"
		}
		if ($ExpectedExit -eq 2 -and $errors -notmatch '^Usage:') {
			throw "Missing usage diagnostic: $Executable $Arguments`n$errors"
		}
	}
	finally {
		$process.Dispose()
	}
}

$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (!(Test-Path -LiteralPath $vswhere)) {
	throw 'Visual Studio Installer discovery tool not found. Install Visual Studio with Desktop development with C++.'
}
$vs = & $vswhere -latest -prerelease -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if ($LASTEXITCODE -ne 0 -or !$vs) {
	throw 'No Visual Studio installation with C++ tools found. Check Desktop development with C++ in Visual Studio Installer.'
}
$msbuild = Join-Path $vs 'MSBuild\Current\Bin\MSBuild.exe'
$target = if ($Platform -eq 'ARM64') { 'aarch64-pc-windows-msvc' } else { 'x86_64-pc-windows-msvc' }
$baseline = "Circle area: 28.27`nRectangle area: 12.00"
$descriptions = "Large circle: 101`nStandard circle: 100`nSquare: 4x4`nRectangle: 4x3`nTriangle: base 4, height 3"
$svg = '<circle r="3" />' + "`n" + '<rect width="4" height="3" />'
$triangle = 'Triangle area: 6.00'
$all = "$baseline`n$descriptions`n$svg`n$triangle"
$count = 0

Push-Location $PSScriptRoot
try {
	foreach ($configuration in @('Debug', 'Release')) {
		& $msbuild '.\Polymorphism.slnx' /restore /m /nologo /v:minimal "/p:Configuration=$configuration" "/p:Platform=$Platform"
		if ($LASTEXITCODE -ne 0) { throw "$configuration/$Platform solution build failed." }

		foreach ($stage in @('Problem', 'Guided', 'Solution')) {
			$number = @{ Problem = '01'; Guided = '02'; Solution = '03' }[$stage]
			$profile = if ($configuration -eq 'Debug') { 'debug' } else { 'release' }
			$executables = @(
				(Join-Path $PSScriptRoot "CSharp\$number-$stage\bin\$Platform\$configuration\net10.0\CSharp.$stage.exe"),
				(Join-Path $PSScriptRoot "artifacts\Cpp\$Platform\$configuration\Cpp.$stage\Cpp.$stage.exe"),
				(Join-Path $PSScriptRoot "Rust\target\$target\$profile\rust-$($stage.ToLowerInvariant()).exe")
			)
			foreach ($executable in $executables) {
				$expected = $baseline
				Assert-Example -Executable $executable -ExpectedOutput $expected
				Assert-Example -Executable $executable -Arguments '--verify'
				Assert-Example -Executable $executable -ExpectedOutput $expected
				foreach ($invalid in @(@('--unknown'), @('--verify', 'extra'), @('--lesson', '0'), @('--lesson', '5'), @('--lesson'))) {
					Assert-Example -Executable $executable -Arguments $invalid -ExpectedExit 2
				}
				if ($stage -eq 'Solution') {
					$examples = @($baseline, $descriptions, $svg, $triangle)
					for ($lesson = 1; $lesson -le 4; $lesson++) {
						Assert-Example -Executable $executable -Arguments @('--lesson', "$lesson") -ExpectedOutput $examples[$lesson - 1]
					}
					Assert-Example -Executable $executable -Arguments '--all' -ExpectedOutput $all
				}
				$count++
			}
		}
	}

	Push-Location '.\Rust'
	try {
		cargo +stable test --workspace --locked --target $target
		if ($LASTEXITCODE -ne 0) { throw 'Rust tests failed.' }
		cargo +stable fmt --all -- --check
		if ($LASTEXITCODE -ne 0) { throw 'Rust formatting check failed.' }
		cargo +stable clippy --workspace --all-targets --locked --target $target -- -D warnings
		if ($LASTEXITCODE -ne 0) { throw 'Rust Clippy check failed.' }
	}
	finally {
		Pop-Location
	}
}
finally {
	Pop-Location
}

Write-Output "PASS: $count stage/configuration combinations; exact output, verification, selectors, invalid arguments, Rust tests, formatting, and Clippy."