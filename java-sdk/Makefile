java_run: lib
	cd src/ && javac HelloWorld.java && java -Djava.library.path=../java-sdk/target/debug/ HelloWorld

.PHONY: lib

javah:
	javah HelloWorld

lib:
	cd java-sdk && cargo build