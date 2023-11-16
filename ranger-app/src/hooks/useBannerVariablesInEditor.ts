import {type editor} from 'monaco-editor';
import {useTranslation} from 'react-i18next';
import {type BannerVariable} from 'src/models/exercise';

export const useBannerVariablesInEditor
= (editorInstance: editor.IStandaloneCodeEditor | undefined) => {
  const {t} = useTranslation();

  const bannerVariables: BannerVariable[] = [
    {name: '{{exerciseName}}', content: t('emails.variables.exerciseName')},
    {name: '{{deploymentName}}', content: t('emails.variables.deploymentName')},
    {name: '{{username}}', content: t('emails.variables.username')},
  ];

  const insertVariable = (variable: string) => {
    if (editorInstance) {
      const position = editorInstance.getPosition();
      if (!position) {
        return;
      }

      editorInstance.executeEdits('', [{
        range: {
          startLineNumber: position.lineNumber,
          startColumn: position.column,
          endLineNumber: position.lineNumber,
          endColumn: position.column,
        },
        text: variable,
      }]);
    }
  };

  return {bannerVariables, insertVariable};
};
