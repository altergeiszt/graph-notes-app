import { useMemo } from 'react';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import remarkRehype from 'remark-rehype';
import rehypeKatex from 'rehype-katex';
import rehypeHighlight from 'rehype-highlight';
import rehypeStringify from 'rehype-stringify';
import 'katex/dist/katex.min.css';

const processor = unified()
  .use(remarkParse)
  .use(remarkGfm)
  .use(remarkMath)
  .use(remarkRehype, { allowDangerousHtml: false })
  .use(rehypeKatex)
  .use(rehypeHighlight)
  .use(rehypeStringify);

interface PreviewProps {
  content: string;
}

export function Preview({ content }: PreviewProps) {
  const html = useMemo(() => {
    try {
      return String(processor.processSync(content));
    } catch {
      return '<p style="color:red">Render error</p>';
    }
  }, [content]);

  return (
    <div
      className="prose dark:prose-invert max-w-none p-4 overflow-auto h-full"
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
}
