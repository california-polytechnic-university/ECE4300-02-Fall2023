`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 02/20/2023 06:22:59 PM
// Design Name: 
// Module Name: first_sseg_driver
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


module first_sseg_driver
    #(parameter N = 4)
    (
    input logic [2:0] active_digit,
    input logic [N-1:0] num,
    input logic DP_ctrl,
    input logic en,
    output logic [7:0] AN,
    output logic [6:0] sseg,
    output logic DP
    );
    
    logic [7:0] a;
    assign DP = ~DP_ctrl;
    
    decoder_generic #(.N(3)) dec1 (
        .w(active_digit),
        .en(en),
        .y(a)
    );
    
    assign AN = ~a;
        
    hex2sseg h1 (
        .hex(num),
        .sseg(sseg)
    );
    
endmodule
