# PowerShell installer for gh-templates
param(
    [string]$InstallDir = "$env:USERPROFILE\bin"
)

$repo = "RafaelJohn9/gh-templates"
$binaryName = "gh-templates.exe"

function Get-LatestVersion {
    Write-Host "Fetching latest version..." -ForegroundColor Green
    try {
        $releaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -ErrorAction Stop
        $version = $releaseInfo.tag_name
        if ([string]::IsNullOrEmpty($version)) {
            throw "Could not determine latest version"
        }
        Write-Host "Latest version: $version" -ForegroundColor Cyan
        return $version
    }
    catch {
        Write-Error "Failed to fetch latest version: $($_.Exception.Message)"
        exit 1
    }
}

function Detect-Platform {
    $arch = $env:PROCESSOR_ARCHITECTURE
    $target = ""

    switch ($arch) {
        "AMD64" { 
            $target = "x86_64-pc-windows-gnu" 
        }
        "ARM64" { 
            # You may need to verify this target exists in your releases
            Write-Warning "ARM64 detected - using x86_64 binary (may need adjustment)"
            $target = "x86_64-pc-windows-gnu"  # Most compatible option
        }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }

    Write-Host "Detected platform: $target" -ForegroundColor Cyan
    return $target
}

function Download-Binary {
    param(
        [string]$Version,
        [string]$Target
    )

    $url = "https://github.com/$repo/releases/download/$Version/gh-templates-$Target.exe"
    $outputPath = Join-Path $env:TEMP "$binaryName"

    Write-Host "Downloading $binaryName for $Target..." -ForegroundColor Green
    Write-Host "From: $url" -ForegroundColor Gray
    
    try {
        Invoke-WebRequest -Uri $url -OutFile $outputPath -ErrorAction Stop
        return $outputPath
    }
    catch {
        Write-Error "Failed to download binary: $($_.Exception.Message)"
        Write-Host "Please check if the binary exists for your platform at:" -ForegroundColor Yellow
        Write-Host "https://github.com/$repo/releases/tag/$Version" -ForegroundColor Gray
        exit 1
    }
}

function Install-Binary {
    param(
        [string]$BinaryPath
    )

    # Create install directory
    if (!(Test-Path $InstallDir)) {
        try {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
            Write-Host "Created directory: $InstallDir" -ForegroundColor Cyan
        }
        catch {
            Write-Error "Failed to create directory $InstallDir : $($_.Exception.Message)"
            exit 1
        }
    }

    # Move binary to install directory
    $destination = Join-Path $InstallDir $binaryName
    try {
        Move-Item -Path $BinaryPath -Destination $destination -Force
        Write-Host "Installed $binaryName to $InstallDir" -ForegroundColor Green
        return $destination
    }
    catch {
        Write-Error "Failed to move binary to $InstallDir : $($_.Exception.Message)"
        exit 1
    }
}

function Add-ToPathNotice {
    $pathExists = $env:PATH -split ';' | Where-Object { $_ -eq $InstallDir }
    
    if (-not $pathExists) {
        Write-Host "`n Please add $InstallDir to your PATH:" -ForegroundColor Yellow
        Write-Host "   Run this command to add it temporarily:" -ForegroundColor Gray
        Write-Host "   `$env:PATH += ';$InstallDir'" -ForegroundColor Gray
        Write-Host "`n   To add permanently, add this to your PowerShell profile:" -ForegroundColor Gray
        Write-Host "   [Environment]::SetEnvironmentVariable('PATH', `$env:PATH + ';$InstallDir', 'User')" -ForegroundColor Gray
    } else {
        Write-Host "`n Installation complete! Run 'gh-templates --help' to get started." -ForegroundColor Green
    }
}

# Main execution
try {
    Write-Host " Installing gh-templates..." -ForegroundColor Magenta
    $version = Get-LatestVersion
    $target = Detect-Platform
    $binaryPath = Download-Binary -Version $version -Target $target
    $installedPath = Install-Binary -BinaryPath $binaryPath
    Add-ToPathNotice
}
catch {
    Write-Error "Installation failed: $($_.Exception.Message)"
    exit 1
}