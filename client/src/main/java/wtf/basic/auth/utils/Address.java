package wtf.basic.auth.utils;

import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;
import wtf.basic.auth.Main;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

/**
 * Created: 1/19/2025
 */
public class Address {
    /**
     * Fetches the actual public IP of the machine by calling an external service.
     *
     * @return The public IP address.
     * @throws IOException If the HTTP request fails.
     */
    public static String getPublicIP() throws IOException {
        String ipifyUrl = "https://api.ipify.org?format=text";

        Request request = new Request.Builder()
                .url(ipifyUrl)
                .build();

        try (Response response = Main.getCLIENT().newCall(request).execute()) {
            if (response.isSuccessful()) {
                return response.body().string();
            } else {
                throw new IOException("Failed to get public IP: " + response.code());
            }
        }
    }
}
