public class HelloWorld {
    private static native String hello();

    static {
        try {
            System.loadLibrary("java_sdk");
        } catch (UnsatisfiedLinkError e) {
            System.out.println("Error loading native library: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        String output = HelloWorld.hello();
        System.out.println(output);
    }
}
