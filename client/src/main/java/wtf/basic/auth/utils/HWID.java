package wtf.basic.auth.utils;

import wtf.basic.auth.hwid.HwidKit;
import wtf.basic.auth.hwid.SystemSpecification;

/**
 * Created: 1/19/2025
 */

public class HWID {
    public static String getHWID() {
        HwidKit hwidKit = new HwidKit();
        String generatedHwid = hwidKit.generateIdentifier (
                SystemSpecification.OS_NAME,
                SystemSpecification.OS_ARCHITECTURE,
                SystemSpecification.OS_VERSION,
                SystemSpecification.AVAILABLE_PROCESSORS
        );

        return generatedHwid;
    }
}