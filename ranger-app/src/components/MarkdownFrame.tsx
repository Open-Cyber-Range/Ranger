import type React from 'react';
import {useEffect, useRef} from 'react';
import ReactDOM from 'react-dom';
import ReactMarkdown from 'react-markdown';
import gfm from 'remark-gfm';

const MarkdownFrame: React.FC<{content: string}> = ({content}) => {
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    const renderMarkdown = () => {
      const iframe = iframeRef.current;
      const cssLink = document.createElement('link');
      cssLink.href = '/gfm.min.css';
      cssLink.rel = 'stylesheet';
      cssLink.type = 'text/css';
      const doc = iframe?.contentDocument;
      const div = doc?.createElement('div');
      if (doc && div) {
        doc.body.innerHTML = '';
        doc.body.append(div);
        doc.head.append(cssLink);
        ReactDOM.render(
          <ReactMarkdown remarkPlugins={[gfm]}>{content}</ReactMarkdown>,
          div,
        );
        iframe.style.height = `${doc.body.scrollHeight}px`;
      }
    };

    const iframe = iframeRef.current;
    iframe?.addEventListener('load', renderMarkdown);

    return () => {
      iframe?.removeEventListener('load', renderMarkdown);
    };
  }, [content]);

  return (
    <iframe
      ref={iframeRef}
      className='flex-grow'
      style={{width: '100%'}}
      sandbox='allow-same-origin'
    />
  );
};

export default MarkdownFrame;
