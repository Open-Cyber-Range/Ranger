import type {ReactNode} from 'react';
import React from 'react';
import styled from 'styled-components';

const WidthBox = styled.div`
  display: flex;
  justify-content: center;
`;

const PageWrapper = styled.div`
  padding-top: 2rem;
  max-width: 60rem;
  width: 100%;
`;

const PageHolder = ({children}: {children: ReactNode}) => (
  <WidthBox>
    <PageWrapper>
      {children}
    </PageWrapper>
  </WidthBox>
);

export default PageHolder;
