package wtf.basic.auth.objects;

/**
 * Created: 1/19/2025
 */
public class ApiResponse {
    private final boolean success;
    private final String body;
    private final int statusCode;

    public ApiResponse(boolean success, String body, int statusCode) {
        this.success = success;
        this.body = body;
        this.statusCode = statusCode;
    }

    public boolean isSuccess() {
        return success;
    }

    @Override
    public String toString() {
        return String.format("ApiResponse{success=%b, body='%s', statusCode=%d}", success, body, statusCode);
    }
}