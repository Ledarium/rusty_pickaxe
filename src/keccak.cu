#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h> 
#include <time.h> 
#include <math.h> 


void runBenchmarks();
int gcd(int a, int b);


int clock_speed;
int number_multi_processors;
int number_blocks;
int number_threads;
int max_threads_per_mp;


#define MSG_SIZE 16
#define THREADS_PER_BLOCK 512

const int digest_size = 256;
const int digest_size_bytes = digest_size / 8;

uint64_t h_pre_state[25];    
__device__ uint64_t d_pre_state[25];    
__device__ uint64_t state[25];    

__device__ uint8_t d_message[65];    
__device__ uint64_t d_res_nonces[1];    

// cudaEvent_t start, stop;
#define ROTL64(x, y) (((x) << (y)) | ((x) >> (64 - (y))))


__device__ const uint64_t RC[24] = {
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
    0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
    0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
    0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 
    0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
    0x8000000000008080, 0x0000000080000001, 0x8000000080008008
};

__device__ const int r[24] = {
    1,  3,  6,  10, 15, 21, 28, 36, 45, 55, 2,  14, 
    27, 41, 56, 8,  25, 43, 62, 18, 39, 61, 20, 44
};

__device__ const int piln[24] = {
    10, 7,  11, 17, 18, 3, 5,  16, 8,  21, 24, 4, 
    15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6,  1 
};

__device__ void keccakF(){
    uint64_t temp, C[5];
	int j;
	
    for (int i = 0; i < 24; i++) {
        // Theta
		// for i = 0 to 5 
		//    C[i] = state[i] ^ state[i + 5] ^ state[i + 10] ^ state[i + 15] ^ state[i + 20];
		C[0] = state[0] ^ state[5] ^ state[10] ^ state[15] ^ state[20];
		C[1] = state[1] ^ state[6] ^ state[11] ^ state[16] ^ state[21];
		C[2] = state[2] ^ state[7] ^ state[12] ^ state[17] ^ state[22];
		C[3] = state[3] ^ state[8] ^ state[13] ^ state[18] ^ state[23];
		C[4] = state[4] ^ state[9] ^ state[14] ^ state[19] ^ state[24];
		
		// for i = 0 to 5
		//     temp = C[(i + 4) % 5] ^ ROTL64(C[(i + 1) % 5], 1);
		//     for j = 0 to 25, j += 5
		//          state[j + i] ^= temp;
		temp = C[4] ^ ROTL64(C[1], 1);
		state[0] ^= temp;
		state[5] ^= temp;
		state[10] ^= temp;
		state[15] ^= temp;
		state[20] ^= temp;
		
		temp = C[0] ^ ROTL64(C[2], 1);
		state[1] ^= temp;
		state[6] ^= temp;
		state[11] ^= temp;
		state[16] ^= temp;
		state[21] ^= temp;
		
		temp = C[1] ^ ROTL64(C[3], 1);
		state[2] ^= temp;
		state[7] ^= temp;
		state[12] ^= temp;
		state[17] ^= temp;
		state[22] ^= temp;
		
		temp = C[2] ^ ROTL64(C[4], 1);
		state[3] ^= temp;
		state[8] ^= temp;
		state[13] ^= temp;
		state[18] ^= temp;
		state[23] ^= temp;
		
		temp = C[3] ^ ROTL64(C[0], 1);
		state[4] ^= temp;
		state[9] ^= temp;
		state[14] ^= temp;
		state[19] ^= temp;
		state[24] ^= temp;
		
        // Rho Pi
		// for i = 0 to 24
		//     j = piln[i];
		//     C[0] = state[j];
		//     state[j] = ROTL64(temp, r[i]);
		//     temp = C[0];
		temp = state[1];
		j = piln[0];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[0]);
		temp = C[0];
		
		j = piln[1];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[1]);
		temp = C[0];
		
		j = piln[2];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[2]);
		temp = C[0];
		
		j = piln[3];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[3]);
		temp = C[0];
		
		j = piln[4];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[4]);
		temp = C[0];
		
		j = piln[5];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[5]);
		temp = C[0];
		
		j = piln[6];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[6]);
		temp = C[0];
		
		j = piln[7];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[7]);
		temp = C[0];
		
		j = piln[8];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[8]);
		temp = C[0];
		
		j = piln[9];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[9]);
		temp = C[0];
		
		j = piln[10];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[10]);
		temp = C[0];
		
		j = piln[11];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[11]);
		temp = C[0];
		
		j = piln[12];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[12]);
		temp = C[0];
		
		j = piln[13];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[13]);
		temp = C[0];
		
		j = piln[14];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[14]);
		temp = C[0];
		
		j = piln[15];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[15]);
		temp = C[0];
		
		j = piln[16];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[16]);
		temp = C[0];
		
		j = piln[17];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[17]);
		temp = C[0];
		
		j = piln[18];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[18]);
		temp = C[0];
		
		j = piln[19];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[19]);
		temp = C[0];
		
		j = piln[20];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[20]);
		temp = C[0];
		
		j = piln[21];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[21]);
		temp = C[0];
		
		j = piln[22];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[22]);
		temp = C[0];
		
		j = piln[23];
		C[0] = state[j];
		state[j] = ROTL64(temp, r[23]);
		temp = C[0];

        //  Chi
		// for j = 0 to 25, j += 5
		//     for i = 0 to 5
		//         C[i] = state[j + i];
		//     for i = 0 to 5
		//         state[j + 1] ^= (~C[(i + 1) % 5]) & C[(i + 2) % 5];
		C[0] = state[0];
		C[1] = state[1];
		C[2] = state[2];
		C[3] = state[3];
		C[4] = state[4];
			
		state[0] ^= (~C[1]) & C[2];
		state[1] ^= (~C[2]) & C[3];
		state[2] ^= (~C[3]) & C[4];
		state[3] ^= (~C[4]) & C[0];
		state[4] ^= (~C[0]) & C[1];
		
		C[0] = state[5];
		C[1] = state[6];
		C[2] = state[7];
		C[3] = state[8];
		C[4] = state[9];
			
		state[5] ^= (~C[1]) & C[2];
		state[6] ^= (~C[2]) & C[3];
		state[7] ^= (~C[3]) & C[4];
		state[8] ^= (~C[4]) & C[0];
		state[9] ^= (~C[0]) & C[1];
		
		C[0] = state[10];
		C[1] = state[11];
		C[2] = state[12];
		C[3] = state[13];
		C[4] = state[14];
			
		state[10] ^= (~C[1]) & C[2];
		state[11] ^= (~C[2]) & C[3];
		state[12] ^= (~C[3]) & C[4];
		state[13] ^= (~C[4]) & C[0];
		state[14] ^= (~C[0]) & C[1];

		C[0] = state[15];
		C[1] = state[16];
		C[2] = state[17];
		C[3] = state[18];
		C[4] = state[19];
			
		state[15] ^= (~C[1]) & C[2];
		state[16] ^= (~C[2]) & C[3];
		state[17] ^= (~C[3]) & C[4];
		state[18] ^= (~C[4]) & C[0];
		state[19] ^= (~C[0]) & C[1];
		
		C[0] = state[20];
		C[1] = state[21];
		C[2] = state[22];
		C[3] = state[23];
		C[4] = state[24];
			
		state[20] ^= (~C[1]) & C[2];
		state[21] ^= (~C[2]) & C[3];
		state[22] ^= (~C[3]) & C[4];
		state[23] ^= (~C[4]) & C[0];
		state[24] ^= (~C[0]) & C[1];
		
        //  Iota
        state[0] ^= RC[i];
    }
}

__global__ void g_set_block() {
    printf("do keccakf\n");
    keccakF();
    printf("done keccakf\n");
}

extern "C" __host__ void h_set_block(const uint8_t *bytes) {
    //get 17 bytes of data, keccakF them
    int rsize = 136;
    int rsize_byte = rsize/8;
    
    memset(h_pre_state, 0, sizeof(h_pre_state));

    for (int i = 0; i < rsize_byte; i++) {
        h_pre_state[i] ^= ((uint64_t *) bytes)[i];
    }
    /*
    printf("pre_state\n");
    for (int i = 0; i < rsize_byte; i++) {
        printf("%d|",h_pre_state[i]);
    }
    printf("\n");
    */
	cudaMemcpyToSymbol(h_pre_state, d_pre_state, 17*sizeof(uint64_t), 0, cudaMemcpyHostToDevice);
	g_set_block<<<1,1>>>();
}

__global__ void g_mine(uint64_t end_nonce, uint64_t target) {
    // get last 64 bytes, pad them and keccakF
    uint8_t temp[144];
    int rsize = 136;
    int rsize_byte = rsize/8;
    int message_len = 64;
    d_res_nonces[0] = UINT64_MAX;
    //printf("do memcpy pre_state\n");
    memcpy(state, d_pre_state, 25);
    // last block and padding
    memcpy(temp, d_message, message_len);
    uint64_t* saltL = ((uint64_t *) temp)+3;
	int tid = threadIdx.x + (blockIdx.x * blockDim.x);
	int num_threads = blockDim.x * gridDim.x;
    //printf("tid %d threads %d\n", tid, num_threads);
    uint64_t start_nonce = *saltL;
    //printf("starting from %d\n", start_nonce+tid);
	while (end_nonce - start_nonce > 0) 
    {
        *saltL = start_nonce+tid;
        temp[message_len] = 0x01;
        memset(temp + message_len, 0, rsize - message_len);
        temp[rsize - 1] |= 0x80;

        for (int i = 0; i < rsize_byte; i++) {
            state[i] ^= ((uint64_t *) temp)[i];
        }
        keccakF();
        if (state[0] <= target) {
            d_res_nonces[0] = start_nonce+tid;
            printf("Cur salt %d|\n", *saltL);
        }
        start_nonce +=  num_threads;
    }
}
extern "C" __host__ uint32_t h_gpu_init(){
    cudaDeviceProp device_prop;
    int device_count, block_size;

    cudaGetDeviceCount(&device_count);
    /*
    if (device_count != 1) {
        exit(EXIT_FAILURE);
    }
    */

    if (cudaGetDeviceProperties(&device_prop, 0) != cudaSuccess) {
        exit(EXIT_FAILURE);
    } 

    number_threads = device_prop.maxThreadsPerBlock;
    number_multi_processors = device_prop.multiProcessorCount;
    max_threads_per_mp = device_prop.maxThreadsPerMultiProcessor;
    block_size = (max_threads_per_mp / gcd(max_threads_per_mp, number_threads));
    number_threads = max_threads_per_mp / block_size;
    number_blocks = block_size * number_multi_processors;
    clock_speed = (int) (device_prop.memoryClockRate * 1000 * 1000);    
    return number_threads;
}

int gcd(int a, int b) {
    return (a == 0) ? b : gcd(b % a, a);
}

extern "C" __host__ uint64_t h_mine(const uint8_t* message, uint32_t end_nonce, uint64_t target, uint32_t block, uint32_t grid) {
	//dim3 dimBlock(ceil((double)array_size / (double)(512 * 7)));
    dim3 dimBlock(block);
  	dim3 dimGrid(grid);
    uint64_t res_nonces[1] = {UINT64_MAX};

	cudaMemcpy(d_message, message, 64*sizeof(uint8_t), cudaMemcpyHostToDevice); // copy message to device
	g_mine<<<dimBlock, dimGrid>>>(end_nonce, target);
	cudaMemcpy(res_nonces, d_res_nonces, sizeof(uint64_t), cudaMemcpyDeviceToHost); // copy message from device
    //cudaDeviceSynchronize();
    return res_nonces[0];
}

/*
int main()
{
    h_gpu_init();
    uint8_t test[136];
    memset(test, 1, sizeof(test));
    h_set_block(test);
    dim3 dimBlock(1);
  	dim3 dimGrid(1);
    uint32_t rc = h_mine(test, 0, 10);
    printf("\n%d\n", rc);
    cudaDeviceSynchronize();
}
*/
