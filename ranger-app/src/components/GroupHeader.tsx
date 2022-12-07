
import React, {useState} from 'react';
import {Button, H2} from '@blueprintjs/core';
import GroupDialog from 'src/components/GroupDialog';
import styled from 'styled-components';

const GroupHeaderHolder = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 4rem;
`;

const GroupHeader = (
  {onSubmit, dialogTitle, buttonTitle, headerTitle}: {
    onSubmit: (name: string) => void;
    dialogTitle: string;
    buttonTitle: string;
    headerTitle: string;
  },
) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <GroupHeaderHolder>
        <H2>{headerTitle}</H2>
        <Button
          large
          icon='add'
          intent='success'
          text={buttonTitle}
          onClick={() => {
            setIsOpen(true);
          }}/>
      </GroupHeaderHolder>
      <GroupDialog
        title={dialogTitle}
        isOpen={isOpen}
        onCancel={() => {
          setIsOpen(false);
        }}
        onSumbit={value => {
          setIsOpen(false);
          onSubmit(value);
        }}/>
    </>
  );
};

export default GroupHeader;
