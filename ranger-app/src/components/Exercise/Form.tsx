import type React from 'react';
import {useEffect} from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import {
  Button,
  Callout,
  FormGroup,
  InputGroup,
  Intent,
  MenuItem,
} from '@blueprintjs/core';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import type {Exercise, UpdateExercise} from 'src/models/exercise';
import {type AdGroup} from 'src/models/groups';
import {
  useAdminGetGroupsQuery,
  useAdminUpdateExerciseMutation,
} from 'src/slices/apiSlice';
import Editor from '@monaco-editor/react';
import {useTranslation} from 'react-i18next';
import init, {
  parse_and_verify_sdl as parseAndVerifySDL,
} from '@open-cyber-range/wasm-sdl-parser';
import {Suggest2} from '@blueprintjs/select';
import {MenuItem2} from '@blueprintjs/popover2';
import useResourceEstimation from 'src/hooks/useResourceEstimation';

const ExerciseForm = ({exercise, onContentChange, children}:
{
  exercise: Exercise;
  onContentChange: (isChanged: boolean) => void;
  children?: React.ReactNode;
}) => {
  const {t} = useTranslation();
  const {handleSubmit, control, watch} = useForm<UpdateExercise>({
    defaultValues: {
      name: exercise.name,
      sdlSchema: exercise.sdlSchema ?? '',
      groupName: exercise.groupName ?? '',
    },
  });
  const {data: groups} = useAdminGetGroupsQuery();
  const {sdlSchema} = watch();
  const {totalRam, totalCpu, resourceEstimationError} = useResourceEstimation(sdlSchema);

  useEffect(() => {
    const subscription = watch((value, {name, type}) => {
      if (name === 'sdlSchema' && type === 'change') {
        onContentChange(true);
      }
    });
    return () => {
      subscription.unsubscribe();
    };
  }, [watch, onContentChange]);

  const [updateExercise, {isSuccess, error}] = useAdminUpdateExerciseMutation();

  const onSubmit: SubmitHandler<UpdateExercise> = async exerciseUpdate => {
    if (exerciseUpdate.sdlSchema) {
      try {
        parseAndVerifySDL(exerciseUpdate.sdlSchema);
      } catch (error: unknown) {
        if (typeof error === 'string') {
          toastWarning(error);
        } else {
          toastWarning(t('exercises.sdlParsingFail'));
        }

        return;
      }
    }

    await updateExercise({exerciseUpdate, exerciseId: exercise.id});
  };

  useEffect(() => {
    const initializeSdlParser = async () => {
      await init();
    };

    initializeSdlParser()
      .catch(() => {
        toastWarning(t('exercises.sdlParserInitFail'));
      });
  }, [t]);

  useEffect(() => {
    if (isSuccess) {
      toastSuccess(t('exercises.updateSuccess', {
        exerciseName: JSON.stringify(exercise.name),
      }));
    }
  }, [isSuccess, t, exercise.name]);

  useEffect(() => {
    if (error) {
      if ('data' in error) {
        toastWarning(t('exercises.updateFail', {
          errorMessage: JSON.stringify(error.data),
        }));
      } else {
        toastWarning(t('exercises.updateFail'));
      }
    }
  }, [error, t]);

  return (
    <form className='ExerciseForm' onSubmit={handleSubmit(onSubmit)}>
      <Controller
        control={control}
        name='name'
        rules={{required: t('exercises.mustHaveName') ?? ''}}
        render={({
          field: {onChange, onBlur, ref, value}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          return (
            <FormGroup
              labelFor='execise-name'
              labelInfo={t('common.required')}
              helperText={error?.message}
              intent={intent}
              label={t('exercises.name')}
            >
              <InputGroup
                large
                intent={intent}
                value={value}
                inputRef={ref}
                id='execise-name'
                onChange={onChange}
                onBlur={onBlur}
              />
            </FormGroup>
          );
        }}
      />
      <Controller
        control={control}
        name='groupName'
        render={({
          field: {onBlur, ref, value, onChange}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          const activeItem = groups?.find(group => group.name === value);
          return (
            <FormGroup
              labelFor='execise-group'
              helperText={error?.message}
              intent={intent}
              label={t('common.adGroup')}
            >
              <Suggest2<AdGroup>
                inputProps={{
                  onBlur,
                  inputRef: ref,
                  placeholder: '',
                }}
                activeItem={activeItem}
                inputValueRenderer={item => item.name}
                itemPredicate={(query, item) =>
                  item.name.toLowerCase().includes(query.toLowerCase())}
                itemRenderer={(item, {handleClick, handleFocus}) => (
                  <MenuItem2
                    key={item.id}
                    text={item.name}
                    onClick={handleClick}
                    onFocus={handleFocus}
                  />
                )}
                items={groups ?? []}
                noResults={
                  <MenuItem
                    disabled
                    text={t('common.noResults')}
                    roleStructure='listoption'/>
                }
                selectedItem={activeItem}
                onItemSelect={item => {
                  onChange(item.name);
                }}
              />
            </FormGroup>
          );
        }}
      />
      <Controller
        control={control}
        name='sdlSchema'
        render={({
          field: {onChange, value}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          return (
            <FormGroup
              labelFor='sdl-schema'
              helperText={error?.message}
              intent={intent}
              label={
                <Callout intent='primary' title={t('exercises.scenarioSDL') ?? ''}>
                  <span>{t('exercises.easeDevelopment')}</span>
                  <a
                    className='underline text-blue-500'
                    href={
                      'https://documentation.opencyberrange.ee/'
                  + 'docs/sdl-reference-guide/sdl'
                    }
                    target='_blank'
                    rel='noopener noreferrer'
                  >
                    {t('exercises.sdlGuide')}
                  </a>
                </Callout>
              }
            >
              <div className='h-[40vh]'>
                <Editor
                  value={value}
                  defaultLanguage='yaml'
                  onChange={onChange}
                />
              </div>
              <div style={{
                display: 'flex',
                alignItems: 'center',
                gap: '10px',
                marginTop: '20px',
                backgroundColor: '#fafafa',
                padding: '10px',
                boxShadow: '0px 2px 5px rgba(0,0,0,0.1)',
              }}
              >
                {resourceEstimationError ? (
                  <>
                    <h3 style={{fontWeight: 'bold', color: 'red'}}>Error:</h3>
                    <p style={{fontSize: '16px', color: 'red'}}>{resourceEstimationError}</p>
                  </>
                ) : (
                  <>
                    <h3 style={{fontWeight: 'bold', color: '#333'}}>Estimated Resources:</h3>
                    <p style={{fontSize: '16px', color: '#666'}}>
                      Total RAM: {totalRam} GiB, Total CPUs: {totalCpu}
                    </p>
                  </>
                )}
              </div>
            </FormGroup>
          );
        }}
      />
      <div className='flex justify-end mt-[2rem] gap-[2rem]'>
        {children}
        <Button
          large
          type='submit'
          intent='primary'
        >{t('common.submit')}
        </Button>
      </div>
    </form>

  );
};

export default ExerciseForm;
