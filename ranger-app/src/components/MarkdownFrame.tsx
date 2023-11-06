/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/restrict-template-expressions */
/* eslint-disable react/iframe-missing-sandbox */
/* eslint-disable react/prop-types */
import React from 'react';
import ReactDOM from 'react-dom';
import ReactMarkdown from 'react-markdown';
import gfm from 'remark-gfm';

class MarkdownFrame extends React.Component {
  iframeRef = React.createRef();

  componentDidMount() {
    // @ts-expect-error iframeRef unknown
    this.iframeRef.current.addEventListener('load', this.renderMarkdown);
  }

  componentWillUnmount() {
    // @ts-expect-error iframeRef unknown
    this.iframeRef.current.removeEventListener('load', this.renderMarkdown);
  }

  componentDidUpdate(previousProps: {content: any}) {
    // @ts-expect-error content unknown
    if (this.props.content !== previousProps.content) {
      this.renderMarkdown();
    }
  }

  renderMarkdown = () => {
    const iframe = this.iframeRef.current;
    // @ts-expect-error contentDocument unknown
    const doc = iframe.contentDocument;
    const div = doc.createElement('div');
    doc.body.innerHTML = '';
    doc.body.append(div);

    ReactDOM.render(
      // @ts-expect-error content unknown
      <ReactMarkdown remarkPlugins={[gfm]}>{this.props.content}</ReactMarkdown>,
      div,
    );
    // @ts-expect-error iframe unknown
    iframe.style.height = `${doc.body.scrollHeight}px`;
  };

  render() {
    // @ts-expect-error iframeRef unknown
    return <iframe ref={this.iframeRef} style={{width: '100%'}}/>;
  }
}

export default MarkdownFrame;
