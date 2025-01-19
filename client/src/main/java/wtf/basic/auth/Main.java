package wtf.basic.auth;

import com.google.gson.JsonObject;
import lombok.Getter;
import okhttp3.*;
import wtf.basic.auth.objects.ApiResponse;
import wtf.basic.auth.utils.Address;
import wtf.basic.auth.utils.HWID;

import java.io.IOException;
import java.io.PrintWriter;
import java.net.Socket;
import java.net.URL;
import java.util.concurrent.TimeUnit;

/**
 * Created: 1/19/2025
 */
public class Main {
    @Getter
    private static final String API_KEY = "your_secret_api_key_here";
    @Getter
    private static final String SERVER_ADDRESS = "http://127.0.0.1:3030";
    @Getter
    private static final OkHttpClient CLIENT = new OkHttpClient.Builder()
            .connectTimeout(10, TimeUnit.SECONDS)
            .readTimeout(10, TimeUnit.SECONDS)
            .build();

    /*
        change username field ("test") to your persons username
        fabric: MinecraftClient.getInstance().getSession().getUsername();
        forge: Minecraft.getInstance().getSession().getUsername();
        got this from the top of my head so im not sure if these are correct
     */
    public static void main(String[] args) {
        try {
            ApiResponse authResponse = authenticateHwid(HWID.getHWID(), Address.getPublicIP(), "test");
            System.out.println("Authentication response: " + authResponse);

            URL url = new URL(getSERVER_ADDRESS());
            String serverIp = url.getHost();
            int serverPort = url.getPort() != -1 ? url.getPort() : 3030; // change this to ur port, autistic but if ur using domain yea

            try (Socket socket = new Socket(serverIp, serverPort);
                 PrintWriter out = new PrintWriter(socket.getOutputStream(), true)) {

                JsonObject authMessage = new JsonObject();
                authMessage.addProperty("type", "auth");
                authMessage.addProperty("hwid", HWID.getHWID());
                authMessage.addProperty("ip", Address.getPublicIP());
                authMessage.addProperty("username", "test");

                out.println(authMessage);
                System.out.println("Sent message: " + authMessage);
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    /**
     * Authenticate the HWID with the server using the provided information.
     *
     * @param hwid     The hardware ID.
     * @param ip       The public IP address.
     * @param username The username.
     * @return An ApiResponse object.
     * @throws IOException If the request fails.
     */
    private static ApiResponse authenticateHwid(String hwid, String ip, String username) throws IOException {
        JsonObject payload = new JsonObject();
        payload.addProperty("hwid", hwid);
        payload.addProperty("ip", ip);
        payload.addProperty("username", username);

        Request request = new Request.Builder()
                .url(getSERVER_ADDRESS() + "/auth")
                .post(RequestBody.create(payload.toString(), MediaType.get("application/json")))
                .addHeader("X-API-Key", getAPI_KEY())
                .build();

        try (Response response = CLIENT.newCall(request).execute()) {
            return new ApiResponse(response.isSuccessful(), response.body().string(), response.code());
        }
    }
}