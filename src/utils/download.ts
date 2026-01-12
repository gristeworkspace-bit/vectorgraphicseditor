/**
 * Utility function for triggering file downloads
 * This approach works reliably across all browsers including Chrome
 */

export const triggerDownload = (data: string | Blob, filename: string, mimeType: string): void => {
    // 1. Blobの作成
    const blob = typeof data === 'string'
        ? new Blob([data], { type: mimeType })
        : data;

    // 2. 一時的なURLの発行
    const url = URL.createObjectURL(blob);

    // 3. リンクタグの生成と設定
    const link = document.createElement('a');
    link.href = url;
    link.download = filename; // ここでファイル名を強制指定

    // 4. 【重要】DOMに追加しないとChromeで動かない場合がある
    document.body.appendChild(link);

    // 5. クリック発火
    link.click();

    // 6. 後始末
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
};
