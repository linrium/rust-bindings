class Ip {
  public String origin;

  public Ip(String origin) {
    this.origin = origin;
  }
}

class HelloWorld {
  private static native void asyncComputation(HelloWorld callback);

  static {
    System.loadLibrary("java_sdk");
  }

  public static void main(String[] args) {
    System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().getId() + ")");
    asyncComputation(new HelloWorld());
  }

  public void asyncCallback(Ip ip) {
    System.out.println("value = " + ip.origin);
  }
}
