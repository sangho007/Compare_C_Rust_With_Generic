#include <stdio.h>
#include <stdlib.h>

// 아이템 타입을 숨기기 위해 void* 사용
typedef struct {
    void* item; // 어떤 타입의 아이템이든 가리킬 수 있음
} GenericContainer;

// 컨테이너에서 아이템을 가져오는 함수 (반환 타입도 void*)
void* get_item(GenericContainer* container) {
    return container->item;
}

int main() {
    GenericContainer int_container;
    GenericContainer float_container;

    int my_int = 123;
    float my_float = 45.67f; // float 리터럴 명시

    // 컨테이너에 각기 다른 타입의 아이템 주소 저장
    int_container.item = &my_int;
    float_container.item = &my_float;

    // 아이템을 가져올 때, 원래 타입으로 **수동 캐스팅** 필요
    int* retrieved_int_ptr = (int*)get_item(&int_container);
    float* retrieved_float_ptr = (float*)get_item(&float_container);

    printf("가져온 int 값: %d\n", *retrieved_int_ptr);
    printf("가져온 float 값: %.2f\n", *retrieved_float_ptr);

    // 문제점: 잘못된 타입으로 캐스팅해도 컴파일 오류가 발생하지 않음!
    int* wrongly_casted_ptr = (int*)get_item(&float_container);
    printf("잘못 캐스팅된 값: %d\n", *wrongly_casted_ptr); // 정의되지 않은 동작 유발 가능

    return 0;
}