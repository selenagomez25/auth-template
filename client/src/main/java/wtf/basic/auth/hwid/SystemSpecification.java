package wtf.basic.auth.hwid;

/**
 * The SystemSpecification enum represents various system specifications that can be used to generate
 * unique identifiers for user devices. Each enum value corresponds to a specific system specification,
 * such as the operating system name or the system core count.
 *
 * <p>To generate an identifier using the SystemSpecification enum, users can pass one or more enum
 * values to the {@link HwidKit#generateIdentifier(SystemSpecification...)} method. The HwidKit class
 * will then generate a unique identifier based on the specified system specifications.
 *
 * <p>For example:
 *
 * <pre>
 * HwidKit hwidKit = new HwidKit();
 * String identifier = hwidKit.generateIdentifier(
 *     SystemSpecification.OS_NAME,
 *     SystemSpecification.OS_VERSION,
 *     SystemSpecification.USER_NAME
 * );
 * </pre>
 * @author TNT10128
 */
public enum SystemSpecification {

    /** The operating system name. */
    OS_NAME(System.getProperty("os.name")),

    /** The operating system architecture. */
    OS_ARCHITECTURE(System.getProperty("os.arch")),

    /** The operating system version. */
    OS_VERSION(System.getProperty("os.version")),

    /** The user's name. */
    USER_NAME(System.getProperty("user.name")),

    /** The number of available processors on the system. */
    AVAILABLE_PROCESSORS(String.valueOf(Runtime.getRuntime().availableProcessors())),

    /** The processor identifier for Windows systems. */
    WIN_PROCESSOR_IDENTIFIER(System.getenv("PROCESSOR_IDENTIFIER")),

    /** The processor architecture for Windows systems. */
    WIN_PROCESSOR_ARCHITECTURE(System.getenv("PROCESSOR_ARCHITECTURE")),

    /** The processor architecture for 64-bit Windows systems. */
    WIN_PROCESSOR_ARCHITEW6432(System.getenv("PROCESSOR_ARCHITEW6432")),

    /** The number of processors for Windows systems. */
    WIN_NUMBER_OF_PROCESSORS(System.getenv("NUMBER_OF_PROCESSORS"));

    /**
     * The string representation of this system specification.
     * This field is initialized with the value of the system
     * specification when the enum value is created.
     */
    public final String asString;

    SystemSpecification(String asString) {
        this.asString = asString;
    }

    @Override
    public String toString() {
        return asString;
    }

}