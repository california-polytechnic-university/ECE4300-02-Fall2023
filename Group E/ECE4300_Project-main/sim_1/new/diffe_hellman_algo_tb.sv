`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 10/14/2023 03:10:53 PM
// Design Name: 
// Module Name: diffe_hellman_algo_tb
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


module diffe_hellman_algo_tb(

    );
    
    localparam T = 10;
    logic reset;
    logic clk; // generally the 100MHz
    logic [3:0] n, g, x, y;
    logic [7:0] AN;
    logic [6:0] sseg; 
    
    diffe_hellman_algo diffe_call (
        .reset(reset),
        .clk(clk),
        .n(n),
        .g(g),
        .x(x),
        .y(y),
        .AN(AN),
        .sseg(sseg)
    );
    
    always
    begin
        clk = 1'b1;
        #(T / 2);
        clk = 1'b0;
        #(T / 2);
    end
    
    initial
    begin
        reset = 1'b1;
        #(T / 2);
        reset = 1'b0;
        n = 4'b1011;
        g = 4'b0111;
        x = 4'b0011;
        y = 4'b0110;
        #T;
    end

endmodule