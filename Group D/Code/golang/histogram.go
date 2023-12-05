package main

import (
	"fmt"
	"image"
	"image/color"
	"os"
	"path/filepath"

	"gocv.io/x/gocv"
)

func main() {
	startTime := gocv.GetTickCount()

	inputFolder := "../../opencv_dataset/Blurry"

	outputFolder := "histogramDone"
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
			fmt.Printf("Error reading image: %s\n", file)
			continue
		}

		channels := gocv.Split(img)

		histSize := []int{256}
		histRange := []float64{0, 256}
		uniform := true
		histograms := make([]gocv.Mat, len(channels))
		for i, channel := range channels {
			defer channel.Close()
			hist := gocv.NewMat()
			defer hist.Close()
			gocv.CalcHist([]gocv.Mat{channel}, []int{0}, gocv.NewMat(), &hist, histSize, histRange, uniform)
			histograms[i] = hist
		}

		plot := gocv.NewMatWithSize(800, 600, gocv.MatTypeCV8UC3)
		defer plot.Close()
		plot.SetTo(gocv.NewScalar(255, 255, 255, 255))
		colors := []color.RGBA{{255, 0, 0, 0}, {0, 255, 0, 0}, {0, 0, 255, 0}}
		for i, hist := range histograms {
			gocv.Normalize(hist, &hist, 0.0, 600.0, gocv.NormMinMax)
			for j := 1; j < hist.Rows(); j++ {
				gocv.Line(&plot, image.Pt(j-1, plot.Rows()-int(hist.GetFloatAt(j-1, 0))),
					image.Pt(j, plot.Rows()-int(hist.GetFloatAt(j, 0))), colors[i], 2)
			}
		}
		outputFile := filepath.Join(outputFolder, filepath.Base(file))
		if ok := gocv.IMWrite(outputFile, plot); !ok {
			fmt.Printf("Error writing image: %s\n", outputFile)
			continue
		}
		fmt.Printf("Image processed, Histogram saved: %s\n", outputFile)
	}
	endTime := gocv.GetTickCount()
	elapsedTime := (endTime - startTime) / gocv.GetTickFrequency()

	fmt.Printf("Execution Time: %v seconds\n", elapsedTime)
}
