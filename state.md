```mermaid
stateDiagram
    [*] --> Start
    Start --> GenerateCode
    GenerateCode --> CompileCode
    CompileCode --> CompilationSuccess : Компиляция успешна
    CompileCode --> CheckDependencies : Компиляция неудачна
    CheckDependencies --> GenerateDependencies : Требуются зависимости
    GenerateDependencies --> CompileCode
    CheckDependencies --> RewriteCode : Зависимости не требуются
    RewriteCode --> CompileCode
    CompilationSuccess --> GenerateTests
    GenerateTests --> RunTests
    RunTests --> TestsPass : Тесты пройдены
    RunTests --> DecideFix : Тесты не пройдены
    DecideFix --> RewriteCode : Ошибка в коде
    DecideFix --> RewriteTests : Ошибка в тестах
    RewriteCode --> CompileCode
    RewriteTests --> RunTests
    TestsPass --> [*]
 ```   
