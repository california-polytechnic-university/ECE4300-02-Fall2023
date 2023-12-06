`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 10/14/2023 12:00:07 PM
// Design Name: 
// Module Name: diffe_hellman_algo
// Project Name: 
// Target Devices: 
// Tool Versions: 
// Description: 
// 
// Dependencies: 
// 
// Revision:
// Revision 0.01 - File Created
// Additional Comments:
// 
//////////////////////////////////////////////////////////////////////////////////


module diffe_hellman_algo
#(parameter N = 4)(
    input logic reset,
    input logic clk,
    input logic [N - 1: 0] y, x,
    input logic [N - 1: 0] g, n,
    input logic k, s,
    output logic [7:0] AN,
    output logic [6:0] sseg
    );
    
//    logic [N - 1: 0] K_next, K_reg;
//    logic [N - 1: 0] temp_next, temp_reg;
    logic k_db, s_db;
    logic done_tick;
    logic [11:0] bcd_k, bcd_s;
    logic [31: 0] A_k, K1_k, B_s, K2_s;
    logic [4: 0] A_k_mod, B_s_mod, K1_k_mod, K2_s_mod;
//    early_debouncer kb(
//        .sw(k),
//        .reset(!reset),
//        .clk(clk), // genrally the 100MHz
//        .db(k_db)
//    );
    
//    early_debouncer sb(
//        .sw(s),
//        .reset(!reset),
//        .clk(clk), // genrally the 100MHz
//        .db(s_db)
//    );
    
//    ready_check_fsm Acknowledgement(
//    .clk(clk),
//    .reset(!reset),
//    .x_in(g), .y_in(n),
//    .k_r(k_db),
//    .s_r(s_db),
//    .x_out(g_out), .y_out(n_out)
//    );
    
    exponential_function A (
        .base(g),
        .exponent(x),
        .be(A_k)
    );
    assign A_k_mod = A_k % n;
    exponential_function B (
        .base(g),
        .exponent(y),
        .be(B_s)
    );
    assign B_s_mod = B_s % n;
    exponential_function K1 (
        .base(B_s_mod),
        .exponent(x),
        .be(K1_k)
    );
    assign K1_k_mod = K1_k % n;
    exponential_function K2 (
        .base(A_k_mod),
        .exponent(y),
        .be(K2_s)
    );
    assign K2_s_mod = K2_s % n;
//    always_ff @(posedge clk, posedge reset)
//    begin
//        if(reset)
//        begin
//            temp_reg <= 0;
//            K_reg <= 0;
//        end
//        else
//        begin
//            temp_reg <= temp_next;
//            K_reg <= K_next;
//        end
//    end
//    assign JA_O = temp_reg;
    
    bin2bcd b2bcd_K1 (
        .bin(K1_k_mod),
        .bcd(bcd_k)
    );
    
    bin2bcd b2bcd_K2 (
        .bin(K2_s_mod),
        .bcd(bcd_s)
    );
    
    seven_seg_display_driver sseg_driver (
        .en(1'b1),
        .clk(clk),
        .reset(!reset),
        .AN(AN),
        .sseg(sseg),
        .i0({1'b1, bcd_k[3:0], 1'b1}),
        .i1({1'b1, bcd_k[7:4], 1'b1}),
        .i2({1'b1, bcd_k[11:8], 1'b1}),
        .i4({1'b1, bcd_s[3:0], 1'b1}),
        .i5({1'b1, bcd_s[7:4], 1'b1}),
        .i6({1'b1, bcd_s[11:8], 1'b1})
    );
    
endmodule
