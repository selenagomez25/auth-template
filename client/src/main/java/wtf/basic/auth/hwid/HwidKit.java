package wtf.basic.auth.hwid;

/**
 * Created: 1/19/2025
 */

import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.Arrays;

/**
 * The HwidKit class provides a set of methods for generating unique identifiers for user devices.
 * These identifiers can be used to identify devices in a variety of contexts, such as for tracking
 * application usage or for software licensing.
 *
 * <p>The HwidKit class supports generating identifiers based on various specifications, like
 * the operating system or core count. It also supports generating hashes of the generated
 * identifiers using the MD5 algorithm, for added security.
 *
 * <p>To use the HwidKit class, users can pass in the desired specifications for the identifier
 * using the {@link SystemSpecification} enum. The HwidKit class will then generate a unique
 * identifier based on these specifications, and return it as a string.
 *
 * <p>For example:
 *
 * <pre>
 * HwidKit hwidKit = new HwidKit();
 * String identifier = hwidKit.generateIdentifier(
 *     SystemSpecification.OS_NAME,
 *     SystemSpecification.OS_VERSION,
 *     SystemSpecification.AVAILABLE_PROCESSORS
 * );
 * </pre>
 * @author TNT10128
 */
public class HwidKit {

    /**
     * Generates a unique identifier string based on the given system specifications.
     * The generated identifier is not hashed.
     *
     * @param specsToInclude The system specifications to include in the identifier.
     * @return The unique identifier string.
     */
    public String generateIdentifier(SystemSpecification... specsToInclude) {
        String rawIdentifier = generateRawIdentifier(specsToInclude);
        String hashedIdentifier = getMd5HashOfString(rawIdentifier);
        return hashedIdentifier;
    }

    /**
     * Generates a unique identifier string based on the specs passed in.
     * This method does not hash the generated string.
     *
     * @param specsToInclude The specs to include in the identifier.
     * @return The unhashed generated identifier.
     */
    private String generateRawIdentifier(SystemSpecification... specsToInclude) {
        StringBuilder rawHwidStringBuilder = new StringBuilder();
        Arrays.stream(specsToInclude)
                .map(SystemSpecification::toString)
                .forEach(rawHwidStringBuilder::append);
        return rawHwidStringBuilder.toString();
    }

    /**
     * Generates a hash of the given string using the MD5 algorithm.
     *
     * @param toGetHashOf The string to generate a hash for.
     * @return The MD5 hash of the given string.
     */
    public String getMd5HashOfString(String toGetHashOf) {
        try {
            MessageDigest md5MessageDigest = MessageDigest.getInstance("MD5");
            md5MessageDigest.update(toGetHashOf.getBytes(StandardCharsets.US_ASCII));

            byte[] md5HashBytes = md5MessageDigest.digest();
            BigInteger stolenFromStackOverflow = new BigInteger(1, md5HashBytes);
            String md5HashString = String.format("%032x", stolenFromStackOverflow);

            return md5HashString;
        } catch (Throwable throwable) {
            throwable.printStackTrace();
            return null;
        }
    }

}