import React, {useState} from 'react';
import {Alert, Button, H2} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';

// eslint-disable-next-line @typescript-eslint/comma-dangle
const Header = <T,>(
  {onSubmit, buttonTitle, headerTitle, children, askAlert = false}: {
    onSubmit: (value: T) => void;
    buttonTitle: string;
    headerTitle: string;
    askAlert?: boolean;
    children?: React.ReactElement<{
      onCancel: () => void;
      onSubmit: (value: T) => void;
      title: string;
      isOpen: boolean;
    }, any>;
  },
) => {
  const {t} = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const [isAlertOpen, setIsAlertOpen] = useState(false);

  return (
    <>
      <div className='flex flex-row justify-between mb-16'>
        <H2>{headerTitle}</H2>
        {children && (
          <Button
            large
            icon='add'
            intent='success'
            text={buttonTitle}
            onClick={() => {
              if (askAlert) {
                setIsAlertOpen(true);
              } else {
                setIsOpen(true);
              }
            }}/>
        )}
      </div>
      <Alert
        isOpen={isAlertOpen}
        onConfirm={() => {
          setIsAlertOpen(false);
        }}
      >
        <p>{t('exercises.sdlNotSaved')}</p>
      </Alert>

      {children && React.Children.map(children, child => {
        if (React.isValidElement(child)) {
          return React
            .cloneElement(child, {
              isOpen,
              onCancel() {
                setIsOpen(false);
              },
              onSubmit(value: T) {
                if (askAlert) {
                  setIsAlertOpen(true);
                } else {
                  setIsOpen(false);
                  onSubmit(value);
                }
              },
            });
        }

        return null;
      })}
    </>
  );
};

export default Header;
