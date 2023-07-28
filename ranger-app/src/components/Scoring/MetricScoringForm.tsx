import type React from 'react';
import {
  Button,
  FormGroup,
  Intent,
  NumericInput,
  TextArea,
} from '@blueprintjs/core';
import {useState} from 'react';
import {Controller, useForm} from 'react-hook-form';
import {
  type UpdateDummyManualMetric,
  type DummyManualMetric,
} from 'src/models/manualMetric';
import {toastWarning} from 'src/components/Toaster';
import {useTranslation} from 'react-i18next';

const MetricScoringForm = ({metric, onSubmit}:
{
  metric: DummyManualMetric;
  onSubmit: ({name, maxScore, score}: UpdateDummyManualMetric) => void;
}) => {
  const {handleSubmit, control}
  = useForm<UpdateDummyManualMetric>({
    defaultValues: {
      id: '0',
      name: '',
      score: 0,
    },
  });
  const {t} = useTranslation();
  const [loading, setLoading] = useState(false);
  const handleDownload = async (artifactObjectId: string) => {
    setLoading(true);

    try {
      console.log('Downloading file... artifactObjectId:', artifactObjectId);
      // GET artifact file url
      const response = await fetch('http://localhost:5000/hello.txt');
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = 'hello.txt';
      link.click();
    } catch {
      toastWarning(t('metricScoring.errors.downloadFailed'));
    }

    setLoading(false);
  };

  const onHandleSubmit = (formContent: UpdateDummyManualMetric) => {
    if (onSubmit) {
      if (formContent.score === undefined) {
        toastWarning(t('metricScoring.errors.scoreNotSet'));
      } else {
        onSubmit(formContent);
      }
    }
  };

  const validateScore = (value: number) => {
    if (!value || value < 0 || value > metric.maxScore) {
      return `${t('metricScoring.errors.scoreValue', {maxScore: metric.maxScore})} `;
    }

    return true;
  };

  return (
    <div>
      <form
        className='flex flex-col'
        onSubmit={handleSubmit(onHandleSubmit)}
      >
        <div className='flex mt-4 items-center space-x-2'>
          <Button
            large
            intent='primary'
            disabled={loading}
            onClick={async () => handleDownload(metric.id)}
          >
            {loading ? t('metricScoring.downloadButtonLoading') : t('metricScoring.downloadButton')}
          </Button>

          <TextArea
            readOnly
            placeholder={t('metricScoring.textSubmissionPlaceholder') ?? ''}
            className='text-gray-500 h-96 max-h-40 w-96'
            value={metric.textSubmissionValue}
          />

          <Controller
            control={control}
            name='score'
            rules={{validate: validateScore}}
            render={({
              field: {onChange, onBlur, ref, value}, fieldState: {error},
            }) => {
              const intent = error ? Intent.DANGER : Intent.NONE;
              return (
                <FormGroup
                  helperText={error?.message}
                  intent={intent}
                  labelInfo={`(Max: ${metric.maxScore})`}
                  label='Score'
                >
                  <div className='flex flex-row items-center'>
                    <NumericInput
                      fill
                      large
                      placeholder={t('metricScoring.scorePlaceholder') ?? ''}
                      buttonPosition='none'
                      max={metric.maxScore}
                      intent={intent}
                      value={value}
                      inputRef={ref}
                      id='score'
                      onValueChange={onChange}
                      onBlur={onBlur}
                    />
                    <Button
                      large
                      type='submit'
                      className='m-2'
                      intent='primary'
                    >
                      {t('common.submit')}
                    </Button>
                  </div>
                </FormGroup>
              );
            }}
          />

        </div>

      </form>
    </div>
  );
};

export default MetricScoringForm;
