`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 03/08/2023 11:48:41 AM
// Design Name: 
// Module Name: seven_seg_display_driver
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


module seven_seg_display_driver
#(parameter FINAL_VALUE = 50000)
    (
    input logic en,
    input logic clk,
    input logic reset,
    input logic [5:0] i0, i1, i2, i3, i4, i5, i6 , i7,
    output logic [7:0] AN,
    output logic [6:0] sseg,
    output logic DP
    );
    
    logic d;
    logic [2: 0] Qout;
    logic [5:0] muxo;
    
    timer_parameter #(.FINAL_VALUE(FINAL_VALUE)) timer (
        .clk(clk),
        .reset(reset),
        .enable(en),
        .done(d)
    );
    
    udl_counter #(.BITS(3)) counter(
        .clk(clk),
        .reset(reset),
        .enable(d),
        .up(1'b1),
     //   .load(load),
     //   .D(x),
        .Q(Qout)
    );
    
    mux_8x1_nbit #(.N(6)) muxi (
    .w0(i0),
    .w1(i1),
    .w2(i2),
    .w3(i3),
    .w4(i4),
    .w5(i5),
    .w6(i6),
    .w7(i7),
    .s(Qout),
    .f(muxo)
    );

    
//    decoder_generic dec1 (
//    .w(Qout),
//    .en(muxo[5]),
//    .y(AN)   
//    );
    
    first_sseg_driver #(.N(4)) test (
        .active_digit(Qout),
        .num(muxo[4:1]),
        .en(muxo[5]),
        .AN(AN),
        .sseg(sseg),
        .DP_ctrl(muxo[0]),
        .DP(DP)
    ); 
endmodule
