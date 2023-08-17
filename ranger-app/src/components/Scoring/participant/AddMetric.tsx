import type React from 'react';
import {Button, FileInput, TextArea} from '@blueprintjs/core';
import {useState} from 'react';
import {useTranslation} from 'react-i18next';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import {ARTIFACT_FILETYPE_WHITELIST} from 'src/constants';
import {type NewManualMetric} from 'src/models/manualMetric';
import {
  useParticipantAddMetricMutation,
  useParticipantUploadMetricArtifactMutation,
} from 'src/slices/apiSlice';

const AddNewMetric = ({exerciseId, deploymentId, newManualMetric, metricHasArtifact}:
{exerciseId: string;
  deploymentId: string;
  newManualMetric: NewManualMetric;
  metricHasArtifact: boolean;
}) => {
  const {t} = useTranslation();
  const [addMetric, {isError: isMetricError}] = useParticipantAddMetricMutation();
  const [addArtifact, {isError: isArtifactError}]
   = useParticipantUploadMetricArtifactMutation();
  const [artifactFile, setArtifactFile] = useState<File | undefined>(undefined);
  const [submissionText, setSubmissionText] = useState<string>('');

  const handleAddMetric = async () => {
    try {
      newManualMetric.textSubmission = submissionText;
      await addMetric({
        exerciseId,
        deploymentId,
        newManualMetric,
      }).unwrap().then(async metricId => {
        if (artifactFile) {
          await addArtifact({
            exerciseId,
            deploymentId,
            metricId,
            artifactFile,
          });
        }
      });

      if (!isMetricError && !isArtifactError) {
        toastSuccess(t('metricScoring.newSuccess'));
      } else {
        toastWarning(t('metricScoring.errors.newManualMetricFailed'));
      }
    } catch {
      toastWarning(t('metricScoring.errors.newManualMetricFailed'));
    }
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setArtifactFile(event.target.files[0]);
    }
  };

  return (
    <div className='flex flex-col'>
      <form
        className='flex mt-4 justify-between items-center'
        onSubmit={handleAddMetric}
      >
        <FileInput
          className='ml-4'
          id='artifact'
          inputProps={{accept: ARTIFACT_FILETYPE_WHITELIST}}
          text={artifactFile?.name ?? t('metricScoring.addArtifactPlaceholder') ?? ''}
          buttonText={t('common.browse') ?? ''}
          disabled={!metricHasArtifact}
          onInputChange={handleFileChange}

        />
        <TextArea
          className='w-1/2'
          id='submissionText'
          name='submissionText'
          value={submissionText}
          placeholder={t('metricScoring.addSubmissionText') ?? ''}
          onChange={event => {
            setSubmissionText(event.target.value);
          }}/>
        <Button
          className='mr-4'
          intent='primary'
          text={t('common.submit') ?? ''}
          onClick={handleAddMetric}
        />
      </form>
    </div>
  );
};

export default AddNewMetric;
