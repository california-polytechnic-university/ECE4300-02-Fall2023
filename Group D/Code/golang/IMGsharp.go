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

	inputFolder := "../../opencv_dataset/Blurry"
	outputFolder := "IMGsharpDone"
	if err := os.MkdirAll(outputFolder, os.ModePerm); err != nil {
		fmt.Printf("Error creating output folder: %v\n", err)
		return
	}

	files, err := filepath.Glob(filepath.Join(inputFolder, "*.jpg"))
	if err != nil {
		fmt.Printf("Error reading input folder: %v\n", err)
		return
	}

	for _, file := range files {
		img := gocv.IMRead(file, gocv.IMReadColor)
		if img.Empty() {
			continue
		}

		sharpenKernel := gocv.NewMatWithSize(3, 3, gocv.MatTypeCV32F)
		sharpenKernel.SetFloatAt(0, 0, 0)
		sharpenKernel.SetFloatAt(0, 1, -1)
		sharpenKernel.SetFloatAt(0, 2, 0)
		sharpenKernel.SetFloatAt(1, 0, -1)
		sharpenKernel.SetFloatAt(1, 1, 5)
		sharpenKernel.SetFloatAt(1, 2, -1)
		sharpenKernel.SetFloatAt(2, 0, 0)
		sharpenKernel.SetFloatAt(2, 1, -1)
		sharpenKernel.SetFloatAt(2, 2, 0)

		sharpenedImg := gocv.NewMat()
		gocv.Filter2D(img, &sharpenedImg, gocv.MatTypeCV16S, sharpenKernel, image.Point{-1, -1}, 0, gocv.BorderDefault)

		convertedImg := gocv.NewMat()
		gocv.ConvertScaleAbs(sharpenedImg, &convertedImg, 1.0, 0.0)

		outputFile := filepath.Join(outputFolder, filepath.Base(file))
		if ok := gocv.IMWrite(outputFile, convertedImg); !ok {
			continue
		}
		fmt.Printf("Image harpened and saved: %s\n", outputFile)

	}

	endTime := gocv.GetTickCount()
	elapsedTime := (endTime - startTime) / gocv.GetTickFrequency()

	fmt.Printf("Execution Time: %v seconds\n", elapsedTime)
}
