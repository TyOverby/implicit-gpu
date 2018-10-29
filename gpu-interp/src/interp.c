#define POP() stack[--stack_ptr]
#define PUSH(v) stack[stack_ptr++] = v
#define PEEK() stack[stack_ptr - 1]

__kernel void apply(
    __global float* buffer,
    __global float* consts,
    __global char* program,
    __global float* stack,
    ulong max_stack,
    ulong width,
    ulong instr_length)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float) x;
    float y_s = (float) y;

    size_t stack_ptr = pos * max_stack;

    for (ulong i = 0; i < instr_length; i++) {
        char code = program[i];

        printf("code: %d\n", (int)code);
        switch (code) {
            case 0: {
                printf("x: %f\n", x_s);
                PUSH(x_s);
                printf("stack: %f\n", PEEK());
                printf("stack_ptr: %d\n", stack_ptr);
                break;
            }
            case 1: {
                PUSH(y_s);
                break;
            }
            case 2: {
                printf("z not supported yet");
                PUSH(0.0);
                break;
            }
            case 3: {
                float l = POP();
                float r = POP();
                PUSH(l + r);
                break;
            }
            case 4: {
                float l = POP();
                float r = POP();
                PUSH(l - r);
                break;
            }
            case 5: {
                float l = POP();
                float r = POP();
                PUSH(fmax(l, r));
                break;
            }
            case 6: {
                float l = POP();
                float r = POP();
                PUSH(fmin(l, r));
                break;
            }
            case 7: {
                float v = POP();
                PUSH(fabs(v));
                break;
            }
            case 8: {
                float v = POP();
                PUSH(fabs(v));
                break;
            }
            default: {
                printf("constant: %f\n", consts[code - 9]);
                PUSH(consts[code - 9]);
            }
        }
    }

    printf("pos: %d\n", pos);
    printf("pstack: %f\n", PEEK());
    printf("pstack_ptr: %d\n", stack_ptr);
    buffer[pos] = POP();
}
