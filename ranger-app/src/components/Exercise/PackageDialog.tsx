import type React from 'react';
import {useState} from 'react';
import {
  Button,
  Dialog,
  FormGroup,
  H4,
  HTMLSelect,
} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {type Package} from 'src/models/package';

const PackageDialog = (
  {isOpen, exercisePackages, onClose, onPackageSelect}:
  {
    isOpen?: boolean;
    exercisePackages: Package[];
    onClose: () => void;
    onPackageSelect: (selectedPackage: Package | undefined) => void;
  },
) => {
  const {t} = useTranslation();
  const [selectedPackageName, setSelectedPackageName] = useState('');

  const handleAddClick = () => {
    const selectedPackage = exercisePackages?.find(
      exercisePackage => exercisePackage.name === selectedPackageName);
    onPackageSelect(selectedPackage);
    onClose();
  };

  return isOpen ? (
    <Dialog isOpen={isOpen} onClose={onClose}>
      <div className='bp4-dialog-header'>
        <H4>{t('exercises.package.add')}</H4>
        <Button
          small
          minimal
          icon='cross'
          onClick={() => {
            onClose();
          }}/>
      </div>
      <div className='bp4-dialog-body'>
        <FormGroup
          labelFor='exercise-package'
          label={t('exercises.package.title')}
        >
          <HTMLSelect
            large
            fill
            id='exercise-package'
            value={selectedPackageName}
            onChange={(event: React.ChangeEvent<HTMLSelectElement>) => {
              setSelectedPackageName(event.target.value);
            }}
          >
            <option className='hidden' value=''>{t('exercises.package.placeholder')}</option>
            {exercisePackages?.map(exercisePackage => (
              <option key={exercisePackage.name} value={exercisePackage.name}>
                {exercisePackage.name}
              </option>
            ))}
          </HTMLSelect>
        </FormGroup>
      </div>
      <div className='bp4-dialog-footer'>
        <div className='bp4-dialog-footer-actions'>
          <Button
            large
            intent='primary'
            text={t('common.add')}
            onClick={handleAddClick}
          />
        </div>
      </div>
    </Dialog>
  ) : null;
};

export default PackageDialog;
