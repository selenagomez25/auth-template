package wtf.basic.auth;

import com.google.gson.JsonObject;
import okhttp3.*;
import wtf.basic.auth.objects.ApiResponse;
import wtf.basic.auth.utils.HWID;

import java.io.IOException;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

/**
 * Created: 1/19/2025
 */
public class Main {
    private static final String API_KEY = "your_secret_api_key_here";
    private static final String SERVER_ADDRESS = "http://127.0.0.1:3030";
    private static final OkHttpClient CLIENT =
            new OkHttpClient.Builder()
                    .connectTimeout(10, TimeUnit.SECONDS)
                    .readTimeout(10, TimeUnit.SECONDS)
                    .build();

    public static void main(String[] args) {
        authenticate(
                response -> {
                    System.out.println("Authentication successful!");
                },
                error -> {
                    System.err.println("Authentication failed: " + error);
                });
    }

    private static void authenticate(
            Consumer<ApiResponse> onSuccess, Consumer<String> onFailure) {
        try {
            String publicIP = CLIENT
                    .newCall(new Request.Builder()
                            .url("https://checkip.amazonaws.com")
                            .build())
                    .execute()
                    .body()
                    .string()
                    .trim();

            /*
                change username field ("test") to your persons username
                fabric: MinecraftClient.getInstance().getSession().getUsername();
                forge: Minecraft.getInstance().getSession().getUsername();
                got this from the top of my head so im not sure if these are correct
            */
            JsonObject payload = new JsonObject();
            payload.addProperty("hwid", HWID.getHWID());
            payload.addProperty("ip", publicIP);
            payload.addProperty("username", "test");

            Response response =
                    CLIENT
                            .newCall(new Request.Builder()
                                    .url(SERVER_ADDRESS + "/auth")
                                    .post(RequestBody.create(payload.toString(),
                                            MediaType.get("application/json")))
                                    .addHeader("X-API-Key", API_KEY)
                                    .build())
                            .execute();

            ApiResponse authResponse = new ApiResponse(
                    response.isSuccessful(), response.body().string(), response.code());

            if (authResponse.isSuccess()) {
                onSuccess.accept(authResponse);
            } else {
                onFailure.accept(authResponse.toString());
            }

        } catch (IOException e) {
            onFailure.accept(e.getMessage());
        }
    }
}