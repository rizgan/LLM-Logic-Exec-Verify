```mermaid
stateDiagram-v2
    [*] --> Start
    Start --> Ask_Explanation
    Ask_Explanation --> Generate_Code
    Generate_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded
    state Code_Attempts_Exceeded <<choice>>
    Code_Attempts_Exceeded --> Exit : [Code attempts >= Max]
    Code_Attempts_Exceeded --> Create_Project : [Code attempts < Max]
    Create_Project --> Compile_Code
    Compile_Code --> Compilation_Success
    state Compilation_Success <<choice>>
    Compilation_Success --> Dependencies_Loop : [Compilation success]
    Compilation_Success --> Check_Dependencies_Needed : [Compilation failed]

    Check_Dependencies_Needed --> Dependencies_Needed
    state Dependencies_Needed <<choice>>
    Dependencies_Needed --> Generate_Dependencies : [Dependencies needed]
    Dependencies_Needed --> Rewrite_Code : [No dependencies needed]
    Generate_Dependencies --> Update_Dependencies
    Update_Dependencies --> Increment_Dependencies_Attempts
    Increment_Dependencies_Attempts --> Dependencies_Attempts_Exceeded
    state Dependencies_Attempts_Exceeded <<choice>>
    Dependencies_Attempts_Exceeded --> Reset_Code_Attempts : [Dependencies attempts >= Max]
    Dependencies_Attempts_Exceeded --> Create_Project : [Dependencies attempts < Max]
    Reset_Code_Attempts --> Reset_Code_Counter
    Reset_Code_Counter --> Generate_Code
    Rewrite_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded

    Dependencies_Loop --> Generate_Test_Code
    Generate_Test_Code --> Increment_Test_Attempts
    Increment_Test_Attempts --> Test_Attempts_Exceeded
    state Test_Attempts_Exceeded <<choice>>
    Test_Attempts_Exceeded --> Reset_Dependencies_Attempts : [Test attempts >= Max]
    Test_Attempts_Exceeded --> Create_Project_With_Test : [Test attempts < Max]
    Reset_Dependencies_Attempts --> Reset_Dependencies_Counter
    Reset_Dependencies_Counter --> Dependencies_Loop
    Create_Project_With_Test --> Run_Tests
    Run_Tests --> Tests_Passed
    state Tests_Passed <<choice>>
    Tests_Passed --> End : [Tests passed]
    Tests_Passed --> Fix_Code_Or_Test : [Tests failed]

    Fix_Code_Or_Test --> Fix_Decision
    state Fix_Decision <<choice>>
    Fix_Decision --> Rewrite_Code : [Fix code]
    Fix_Decision --> Rewrite_Test_Code : [Fix test]
    Rewrite_Test_Code --> Increment_Test_Attempts
    Increment_Test_Attempts --> Test_Attempts_Exceeded
    Rewrite_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded

    End --> [*]
    Exit --> [*]
 ```   
