//go:build !debug

package main

import (
	"archive/zip"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

func init() {}

func main() {
	go_homepage := "https://go.dev"
	resp, _ := http.Get(go_homepage + "/dl")
	doc, _ := goquery.NewDocumentFromReader(resp.Body)

	var archive_url string
	var archive_name string
	doc.Find(".toggleVisible").First().Find(".expanded").Find("a").Each(func(i int, s *goquery.Selection) {
		href, _ := s.Attr("href")
		if strings.Contains(href, "windows") && strings.Contains(href, "amd") && strings.Contains(href, "zip") {
			idx := strings.LastIndex(href, "/")
			archive_name = href[idx+1:]
			archive_url = go_homepage + href
		}
	})

	fmt.Printf("Download %v\n", archive_name)
	resp, _ = http.Get(archive_url)
	data, _ := io.ReadAll(resp.Body)
	os.WriteFile(archive_name, data, os.ModePerm)

	fmt.Printf("Extract %v\n", archive_name)
	extract(archive_name, "Go")
}

func extract(zipPath, destDir string) {
	reader, _ := zip.OpenReader(zipPath)
	defer reader.Close()

	for _, file := range reader.File {
		targetPath := filepath.Join(destDir, file.Name)

		if file.FileInfo().IsDir() {
			os.MkdirAll(targetPath, os.ModePerm)
			continue
		}

		os.MkdirAll(filepath.Dir(targetPath), os.ModePerm)

		rc, _ := file.Open()
		defer rc.Close()

		outFile, _ := os.OpenFile(targetPath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
		defer outFile.Close()

		io.Copy(outFile, rc)
	}
}
