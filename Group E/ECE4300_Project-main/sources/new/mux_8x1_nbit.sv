`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 03/08/2023 11:53:52 AM
// Design Name: 
// Module Name: mux_8x1_nbit
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


module mux_8x1_nbit
    #(parameter N = 4) (
    input logic [N - 1: 0] w0, w1, w2, w3, w4, w5, w6, w7,
    input logic [2:0] s,
    output logic [N - 1: 0] f
    );
    
    // Dataflow modeling 
    //-------------------------------------------------------------------
    
    // This works but it's not elegant and hard to understand
//    assign f =  ~s[1] & ~s[0]? w0:
//                ~s[1] & s[0]?  w1:
//                s[1] & ~s[0]?  w2:
//                s[1] & s[0]? w3: 'bx;
                
    // another way to write the above continous assignment
//    assign f = s[1]? (s[0]? w3: w2):(s[0]? w1: w0);   
             
    // Behavioral modeling                
    //-------------------------------------------------------------------
    
    // if, else-if, else (Priority Routing Networks)
//    always @(w0, w1, w2, w3, s)
//    begin
//        if (s == 2'b00)
//            f = w0;
//        else if (s == 2'b01) // this can be s == 1 (decimal)
//            f = w1;
//        else if (s == 2'b10)
//            f = w2;
//        else if (s == 2'b11)
//            f = w3; 
//        else
//            f = 'bx;               
//    end

    // w3 has highest priority below
//    always @(w0, w1, w2, w3, s)
//    begin
//        if (s == 2'b11)
//            f = w3;
//        else if (s == 2'b10) // this can be s == 1 (decimal)
//            f = w2;
//        else if (s == 2'b11)
//            f = w1;
//        else if (s == 2'b00)
//            f = w0; 
//        else
//            f = 'bx;               
//    end    
    
    // case (Multiplexing Networks)
    always @(w0, w1, w2, w3, w4, w5, w6, w7, s)
    begin
        case(s)
            3'b000: f = w0;
            3'b001: f = w1;
            3'b010: f = w2;
            3'b011: f = w3;
            3'b100: f = w4;
            3'b101: f = w5;
            3'b110: f = w6;
            3'b111: f = w7;
            default: f = 'bx;
        endcase
    end
    
endmodule
