import java.lang.annotation.*;

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
public @interface AnyAnnotation {
    int integer() default 10;
    long veryLong();
    float floating();
    double someDouble();
    String string();
    byte[] bytes();
    Class clazz() default Object.class;
}
