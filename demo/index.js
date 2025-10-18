// 告诉 PDF.js 本地 worker 在哪（必须！否则 CORS 报错）
pdfjsLib.GlobalWorkerOptions.workerSrc =
  'pdfjs-dist/build/pdf.worker.min.js';

const url = 'test.pdf';          // 同目录本地文件
const canvasWrap = document.getElementById('canvas-wrap');

// 加载文档
const loadingTask = pdfjsLib.getDocument(url);
loadingTask.promise.then(async pdf => {
  console.log('PDF 页数：', pdf.numPages);

  // 逐页渲染
  for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
    const page = await pdf.getPage(pageNum);
    const viewport = page.getViewport({ scale: 1.0 });

    // 创建 canvas
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    canvas.width = viewport.width;
    canvas.height = viewport.height;
    canvasWrap.appendChild(canvas);

    // 渲染
    await page.render({ canvasContext: ctx, viewport }).promise;
  }
}).catch(err => {
  console.error('加载失败', err);
});