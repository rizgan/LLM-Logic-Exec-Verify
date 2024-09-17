1. Ask write code
2. Compile code
3. If compile with error go to 4, else go to 5
4. Rewrite code by error message, go to 2
5. Run test
6. If test fail go to 7, else go to 7
7. Rewrite code by error test message, go to 2
8. End

State Diagram
```mermaid
stateDiagram-v2
    [*] --> Ask_Write_Code
    Ask_Write_Code --> Compile_Code
    Compile_Code --> Compile_Error_Choice
    state Compile_Error_Choice <<choice>>
    Compile_Error_Choice --> Rewrite_Code_By_Error_Message : Compile Error
    Compile_Error_Choice --> Run_Test : No Error
    Rewrite_Code_By_Error_Message --> Compile_Code
    Run_Test --> Test_Fail_Choice
    state Test_Fail_Choice <<choice>>
    Test_Fail_Choice --> Rewrite_Code_By_Test_Error_Message : Test Fail
    Test_Fail_Choice --> End : Test Pass
    Rewrite_Code_By_Test_Error_Message --> Compile_Code
    End --> [*]
 ```   

Entity diagram

1. Description of code
2. Code
3. Test
4. Result of code compile
5. Result of test
```mermaid
classDiagram
    Code -- Compile_Result
    Code -- Test
    Compile_Result -- Test_Result
```