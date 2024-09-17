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
    Compile_Code --> Compile_Error{Compile Error?}
    Compile_Error --> |Yes| Rewrite_Code_By_Error_Message
    Compile_Error --> |No| Run_Test
    Rewrite_Code_By_Error_Message --> Compile_Code
    Run_Test --> Test_Fail{Test Fail?}
    Test_Fail --> |Yes| Rewrite_Code_By_Test_Error_Message
    Test_Fail --> |No| End
    Rewrite_Code_By_Test_Error_Message --> Compile_Code
    End --> [*]
 ```   

Entity diagram

1. Description of code
2. Code
3. Test
```mermaid
classDiagram
    DescriptionOfCode <|-- Code
    DescriptionOfCode <|-- Test
    DescriptionOfCode : +description
    Code : +code
    Test : +test
```