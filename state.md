```mermaid
stateDiagram-v2
    [*] --> Start
    Start --> Ask_Explanation
    Ask_Explanation --> Generate_Code
    Generate_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded{Code attempts >= Max?}
    Code_Attempts_Exceeded --> |Yes| Exit
    Code_Attempts_Exceeded --> |No| Create_Project
    Create_Project --> Compile_Code
    Compile_Code --> Compilation_Success{Compilation success?}
    Compilation_Success --> |Yes| Dependencies_Loop
    Compilation_Success --> |No| Check_Dependencies_Needed

    Check_Dependencies_Needed --> Dependencies_Needed{Dependencies needed?}
    Dependencies_Needed --> |Yes| Generate_Dependencies
    Dependencies_Needed --> |No| Rewrite_Code
    Generate_Dependencies --> Update_Dependencies
    Update_Dependencies --> Increment_Dependencies_Attempts
    Increment_Dependencies_Attempts --> Dependencies_Attempts_Exceeded{Dependencies attempts >= Max?}
    Dependencies_Attempts_Exceeded --> |Yes| Reset_Code_Attempts
    Dependencies_Attempts_Exceeded --> |No| Create_Project
    Reset_Code_Attempts --> Reset_Code_Counter
    Reset_Code_Counter --> Generate_Code
    Rewrite_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded

    Dependencies_Loop --> Generate_Test_Code
    Generate_Test_Code --> Increment_Test_Attempts
    Increment_Test_Attempts --> Test_Attempts_Exceeded{Test attempts >= Max?}
    Test_Attempts_Exceeded --> |Yes| Reset_Dependencies_Attempts
    Test_Attempts_Exceeded --> |No| Create_Project_With_Test
    Reset_Dependencies_Attempts --> Reset_Dependencies_Counter
    Reset_Dependencies_Counter --> Dependencies_Loop
    Create_Project_With_Test --> Run_Tests
    Run_Tests --> Tests_Passed{Tests passed?}
    Tests_Passed --> |Yes| End
    Tests_Passed --> |No| Fix_Code_Or_Test

    Fix_Code_Or_Test --> Fix_Code{Fix code?}
    Fix_Code --> |Yes| Rewrite_Code
    Fix_Code --> |No| Rewrite_Test_Code
    Rewrite_Test_Code --> Increment_Test_Attempts
    Increment_Test_Attempts --> Test_Attempts_Exceeded
    Rewrite_Code --> Increment_Code_Attempts
    Increment_Code_Attempts --> Code_Attempts_Exceeded

    End --> [*]
    Exit --> [*]
 ```   
