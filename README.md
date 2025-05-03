# Compare_C_Rust_With_Memory_Safety

### Rust의 장점 (PPT 19장 내용 기반)

Rust의 연관 타입 방식은 C의 `void*` 방식에 비해 다음과 같은 장점을 가집니다.

1.  **표현력 높은 추상화 및 타입 안전성**:
    * `Container` 트레이트는 아이템의 구체적인 타입(`i32` 또는 `f32`)을 숨기면서 '아이템을 가질 수 있다'는 공통 속성을 추상화합니다.
    * 각 구현체(`IntContainer`, `FloatContainer`)에서 연관 타입 `Item`을 명시하므로, `get_item`을 호출할 때 반환되는 값의 타입이 무엇인지 컴파일러가 정확히 알고 있습니다.
    * 이로 인해 C 예제에서 발생할 수 있는 **잘못된 타입 캐스팅 오류**를 Rust에서는 컴파일 시점에 원천적으로 방지하여 타입 안전성을 크게 높입니다.
2.  **개발 효율성 및 코드 명확성**:
    * 제네릭한 `Container` 트레이트를 정의하고 각 타입에 맞게 구현함으로써, 코드가 명확해지고 재사용성이 높아집니다.
    * C의 `void*` 방식에서 필요한 수동 캐스팅과 그로 인한 잠재적 오류 검사 로직이 필요 없어 개발이 더 효율적입니다.

결론적으로, 동일한 기능(정수와 실수를 담는 컨테이너)을 구현하더라도 Rust의 연관 타입은 C의 `void*` 방식보다 훨씬 안전하고 명확하며 효율적인 코드를 작성할 수 있게 해줍니다. 이는 19장 9페이지에서 설명하는 Rust의 핵심 장점과 일맥상통합니다.

<img width="439" alt="image" src="https://github.com/user-attachments/assets/8523ab34-5048-4bff-b342-9bfd25e14a90" />

<img width="762" alt="image" src="https://github.com/user-attachments/assets/99e4ae34-0125-4ff1-8351-af35eef83cd9" />
