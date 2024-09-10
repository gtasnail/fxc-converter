@echo off
setlocal enabledelayedexpansion

rem USAGE:
rem 1. Put this batch file and cmd_Decompiler.exe in the same directory
rem 2. Drag and drop multiple files (e.g., water.o400, water.o3179) onto this batch file to process them
rem 3. Output files will be named like water.400.hlsl, water.3179.hlsl
rem 4. For use with Renderdoc:
rem 4.1. Renderdoc -> Tools -> Settings -> Shader Viewer -> Add
rem 4.2. Name: whatever you like
rem 4.3. Tool Type: Custom Tool
rem 4.4. Executable: Choose this batch file instead of cmd_Decompiler.exe
rem 4.5. Command Line: {input_file}
rem 4.6. Input/Output: DXBC/HLSL
rem 5. Renderdoc -> Pipeline State View -> Choose Any Shader Stage
rem 5.1. Edit -> Decompile with ${Name}
rem 6. Modify shader as you wish, and click Refresh button to see the change

:process_files
if "%~1"=="" goto end

rem Extract base name and number from file name
for %%f in ("%~1") do (
    set "baseName=%%~nf"
    set "extension=%%~xf"
    set "number=!extension:~2!"
)

rem Decompile input file
"%~dp0cmd_Decompiler.exe" -D "%~1"

rem Rename the output file
if exist "%~dp1!baseName!.hlsl" (
    move "%~dp1!baseName!.hlsl" "%~dp1!baseName!.!number!.hlsl" > nul
    if exist "%~dp1!baseName!.!number!.hlsl" (
        type "%~dp1!baseName!.!number!.hlsl"
        echo.
        echo Processing completed for: %~nx1
        echo Output file: !baseName!.!number!.hlsl
        echo -------------------------------
    ) else (
        echo Error: Failed to rename output file for %~nx1
        echo -------------------------------
    )
) else (
    echo Error: Decompiled file not found for %~nx1
    echo -------------------------------
)

shift
goto process_files

:end
echo All files processed.
pause