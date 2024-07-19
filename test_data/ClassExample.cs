/// <summary>
/// This is a sample class with varied XML tags.
/// It demonstrates how to use multiple types of documentation comments.
/// </summary>
/// <remarks>
/// This class is part of the sample documentation generation project.
/// </remarks>
/// <example>
/// <code>
/// var sample = new SampleClass();
/// sample.DoSomething();
/// </code>
/// </example>
/// <param name="parameter">This is a sample parameter.</param>
/// <typeparam name="T">This is a sample type parameter.</typeparam>
/// <returns>A sample return value.</returns>
public class PublicClass {
    // Class body
    // Single-line comment inside a class !Comment!
    /* Multi-line comment inside a class !Comment!
       that spans multiple lines. class !Comment!
       class !Comment! */
}

// This is a single-line comment before a class !Comment!
private class PrivateClass {
    // Class body
}

protected class ProtectedClass {
    // Class body
}

internal class InternalClass {
    // Class body
}

public abstract class AbstractClass {
    // Class body
}

public static class StaticClass {
    // Class body
}

public partial class PartialClass {
    // Class body
}

public partial class PartialClass {
    // Class body
}

// This is a comment that mentions class but should not be detected
// public class CommentedOutClass { }