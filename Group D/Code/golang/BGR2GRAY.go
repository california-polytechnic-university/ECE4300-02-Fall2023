package main

import (
	"fmt"
	"os"
	"path/filepath"

	"gocv.io/x/gocv"
)

func main() {
	startTime := gocv.GetTickCount()

	inputFolder := "../../opencv_dataset/Blue"

	outputFolder := "B2GDone"
	if err := os.MkdirAll(outputFolder, os.ModePerm); err != nil {
		fmt.Printf("Error creating output folder: %v\n", err)
		return
	}

	files, err := filepath.Glob(filepath.Join(inputFolder, "*.jpg"))
	if err != nil {
		fmt.Printf("Error reading input folder: %v\n", err)
		return
	}
	for _, filePath := range files {
		filename := filepath.Base(filePath)
		image := gocv.IMRead(filePath, gocv.IMReadColor)
		if image.Empty() {
			fmt.Printf("Error reading image: %s\n", filename)
			os.Exit(1)
		}
		grayImage := gocv.NewMat()
		gocv.CvtColor(image, &grayImage, gocv.ColorBGRToGray)

		outputPath := filepath.Join(outputFolder, filename)
		if ok := gocv.IMWrite(outputPath, grayImage); !ok {
			fmt.Printf("Error writing image: %s\n", filename)
		}
		fmt.Printf("Image converted, new image saved: %s\n", outputPath)
	}

	endTime := gocv.GetTickCount()
	elapsedTime := (endTime - startTime) / gocv.GetTickFrequency()
	fmt.Printf("Execution Time: %v seconds\n", elapsedTime)
}
