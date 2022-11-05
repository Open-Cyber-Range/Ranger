
import React, {useState} from 'react';
import {Button, H2} from '@blueprintjs/core';
import NameDialog from 'src/components/NameDialog';
import styled from 'styled-components';

const HeaderHolder = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 4rem;
`;

const Header = (
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
      <HeaderHolder>
        <H2>{headerTitle}</H2>
        <Button
          large
          icon='add'
          intent='success'
          text={buttonTitle}
          onClick={() => {
            setIsOpen(true);
          }}/>
      </HeaderHolder>
      <NameDialog
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

export default Header;
