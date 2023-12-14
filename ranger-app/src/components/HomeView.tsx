import React, {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {Link} from 'react-router-dom';
import backgroundImage from 'src/assets/cr14_taust.jpg';

const HomeView = ({buttonText, buttonLink}: {buttonText: string; buttonLink: string}) => {
  const {t} = useTranslation();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const img = new Image();
    img.src = backgroundImage;
    img.addEventListener('load', () => {
      setLoading(false);
    });
  }, []);

  if (loading) {
    return (
      <div className='flex justify-center items-center h-screen'>
        <div
          className='animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-blue-500'/>
      </div>
    );
  }

  return (
    <div
      className='flex flex-col h-full justify-center items-center
        bg-home-background bg-cover bg-no-repeat bg-center'
    >
      {!loading && (
        <div
          className='text-center font-bold text-white bg-black bg-opacity-50 p-8 rounded-3xl'
        >
          <h1 className='mb-4 text-7xl tracking-wider uppercase'>{t('appName')}</h1>
          <p className='mb-8 py-4 text-2xl border-y'>{t('OCR')}</p>
          <Link
            to={buttonLink}
            className='inline-block px-8 py-2 text-base text-center text-white no-underline
              bg-transparent border-2 border-white rounded-full hover:bg-white hover:text-black'
          >
            {buttonText}
          </Link>
        </div>
      )}
    </div>
  );
};

export default HomeView;
