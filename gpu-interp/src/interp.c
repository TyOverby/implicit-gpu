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
            case OP_CONSTANT_SMALL: {
                int constant_index = program[i+1];
                i ++;
                PUSH(consts[constant_index]);
                break;
            }
            case OP_X: {
                PUSH(x_s);
                break;
            }
            case OP_Y: {
                PUSH(y_s);
                break;
            }
            case OP_Z: {
                printf("z not supported yet");
                PUSH(0.0);
                break;
            }
            case OP_ADD: {
                float l = POP();
                float r = POP();
                PUSH(l + r);
                break;
            }
            case OP_SUB: {
                float l = POP();
                float r = POP();
                PUSH(l - r);
                break;
            }
            case OP_MAX: {
                float l = POP();
                float r = POP();
                PUSH(fmax(l, r));
                break;
            }
            case OP_MIN: {
                float l = POP();
                float r = POP();
                PUSH(fmin(l, r));
                break;
            }
            case OP_ABS: {
                float v = POP();
                PUSH(fabs(v));
                break;
            }
            case OP_SQRT: {
                float v = POP();
                PUSH(sqrt(v));
                break;
            }
            default: {
                printf("unrecognized opcode: %d\n", code);
            }
        }
    }

    printf("pos: %d\n", pos);
    printf("pstack: %f\n", PEEK());
    printf("pstack_ptr: %d\n", stack_ptr);
    buffer[pos] = POP();
}
