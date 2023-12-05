package main

import (
	"fmt"
	"image"
	"os"
	"path/filepath"

	"gocv.io/x/gocv"
)

func main() {
	startTime := gocv.GetTickCount()

	// Define the folder containing your JPEG images
	inputFolder := "../../opencv_dataset/yolo_objects"

	// Create an output folder to save converted images
	outputFolder := "IMGcropDone"
	if err := os.MkdirAll(outputFolder, os.ModePerm); err != nil {
		fmt.Printf("Error creating output folder: %v\n", err)
		return
	}

	// Iterate through the images in the input folder
	files, err := filepath.Glob(filepath.Join(inputFolder, "*.jpg"))
	if err != nil {
		fmt.Printf("Error reading input folder: %v\n", err)
		return
	}

	for _, file := range files {
		img := gocv.IMRead(file, gocv.IMReadColor)
		if img.Empty() {
			fmt.Println("Error reading image")
			return
		}
		width := int(float64(img.Cols()) * 0.25)
		height := int(float64(img.Rows()) * 0.25)

		// Calculate the starting point for the crop area
		startX := (img.Cols() - width) / 2  // Centered horizontally
		startY := (img.Rows() - height) / 2 // Centered vertically

		// Ensure the crop region fits within the image bounds
		if startX+width > img.Cols() {
			width = img.Cols() - startX
		}
		if startY+height > img.Rows() {
			height = img.Rows() - startY
		}

		// Create a rectangle defining the crop region
		roi := image.Rect(startX, startY, startX+width, startY+height)

		// Crop the image using the region of interest (ROI)
		cropped := img.Region(roi)
		outputFile := filepath.Join(outputFolder, filepath.Base(file))
		if ok := gocv.IMWrite(outputFile, cropped); !ok {
			fmt.Printf("Error writing image: %s\n", outputFile)
			continue
		}
		fmt.Printf("Image cropped, new image saved: %s\n", outputFile)
	}

	endTime := gocv.GetTickCount()
	elapsedTime := (endTime - startTime) / gocv.GetTickFrequency()

	fmt.Printf("Execution Time: %v seconds\n", elapsedTime)
}
