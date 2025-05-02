// 제네릭 컨테이너를 위한 트레이트 정의
trait Container {
    // 연관 타입 'Item' 정의: 컨테이너가 담을 아이템의 타입을 나타냄
    type Item;

    // 아이템을 반환하는 메서드 (연관 타입 사용)
    fn get_item(&self) -> Self::Item;
}

// 위 트레이트를 구현하는 구체적인 컨테이너 타입 (정수용)
struct IntContainer {
    item: i32,
}

impl Container for IntContainer {
    type Item = i32; // 이 컨테이너의 Item 타입은 i32

    fn get_item(&self) -> Self::Item {
        self.item
    }
}

// 위 트레이트를 구현하는 구체적인 컨테이너 타입 (실수용)
struct FloatContainer {
    item: f32,
}

impl Container for FloatContainer {
    type Item = f32; // 이 컨테이너의 Item 타입은 f32

    fn get_item(&self) -> Self::Item {
        self.item
    }
}

fn main() {
    let int_container = IntContainer { item: 123 };
    let float_container = FloatContainer { item: 45.67 };

    // get_item 호출 시 컴파일러는 각 컨테이너의 'Item' 연관 타입을 알고 있음
    let retrieved_int: i32 = int_container.get_item();
    let retrieved_float: f32 = float_container.get_item();

    println!("가져온 i32 값: {}", retrieved_int);
    println!("가져온 f32 값: {:.2}", retrieved_float); // 소수점 2자리까지 출력

    // 타입 안전성: 컴파일러가 타입을 검사하므로 잘못된 타입 할당 불가
    let wrong_assignment: i32 = float_container.get_item(); // 컴파일 오류! expected i32, found f32
}