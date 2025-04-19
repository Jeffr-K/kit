#!/bin/bash

# test-rust.sh
# Rust 테스트를 빠르게 실행하기 위한 스크립트

# 색상 정의
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 함수: 도움말 표시
show_help() {
    echo -e "${BLUE}Rust 테스트 스크립트 사용법:${NC}"
    echo "  ./test-rust.sh compile    - 테스트 컴파일만 수행"
    echo "  ./test-rust.sh run        - 컴파일 없이 테스트 실행"
    echo "  ./test-rust.sh all        - 컴파일 후 테스트 실행"
    echo "  ./test-rust.sh [패턴]      - 특정 테스트만 실행"
    echo "  ./test-rust.sh help       - 도움말 표시"
}

# 함수: 테스트 컴파일
compile_tests() {
    echo -e "${YELLOW}테스트 컴파일 중...${NC}"
    cargo test --no-run
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}컴파일 성공!${NC} 테스트를 실행하려면 './test-rust.sh run'을 실행하세요."
    fi
}

# 함수: 테스트 실행
run_tests() {
    echo -e "${YELLOW}컴파일된 테스트 실행 중...${NC}"
    cargo test -- --nocapture
}

# 함수: 특정 테스트 실행
run_specific_test() {
    echo -e "${YELLOW}'$1' 패턴과 일치하는 테스트 실행 중...${NC}"
    cargo test $1 -- --nocapture
}

# 메인 로직
case "$1" in
    "compile")
        compile_tests
        ;;
    "run")
        run_tests
        ;;
    "all")
        compile_tests && run_tests
        ;;
    "help")
        show_help
        ;;
    "")
        echo -e "${YELLOW}명령어가 지정되지 않았습니다.${NC}"
        show_help
        ;;
    *)
        run_specific_test $1
        ;;
esac
