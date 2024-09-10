@echo off
setlocal enabledelayedexpansion

echo HLSL to CSO Converter
echo =====================

for %%F in (%*) do (
    set "inputFile=%%~nxF"
    set "outputFile=%%~dpnF.cso"
    
    set "shaderType="
    echo !inputFile! | findstr /i "\<vs\>" >nul && set "shaderType=vs"
    echo !inputFile! | findstr /i "\<ps\>" >nul && set "shaderType=ps"
    echo !inputFile! | findstr /i "\<gs\>" >nul && set "shaderType=gs"
    echo !inputFile! | findstr /i "\<hs\>" >nul && set "shaderType=hs"
    echo !inputFile! | findstr /i "\<ds\>" >nul && set "shaderType=ds"
    echo !inputFile! | findstr /i "\<cs\>" >nul && set "shaderType=cs"
    
    if not defined shaderType (
        echo Unable to detect shader type for !inputFile!.
        set /p "shaderType=Please enter shader type (vs, ps, gs, hs, ds, cs): "
    )
    
    if defined shaderType (
        echo Converting !inputFile! to !outputFile!...
        fxc /nologo /T !shaderType!_5_0 /E main /Fo "!outputFile!" "%%F"
        if !errorlevel! equ 0 (
            echo Conversion successful.
        ) else (
            echo Conversion failed. Error code: !errorlevel!
        )
    ) else (
        echo Skipping !inputFile! due to unknown shader type.
    )
    echo.
)

echo All conversions completed.

:end
echo.
echo Press any key to exit...
pause