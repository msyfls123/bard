import './css/index.styl'

import wasm from '../Cargo.toml';

const cos = new COS({
  getAuthorization: async (options, callback) => {
    const url = 'https://void.ebichu.cc/sts';
    let data;
    try {
      data = await fetch(url).then(res => res.json());
    } catch {}
    const { credentials, startTime, expiredTime } = data || {};
    if (!(data && credentials)) {
      return console.error('data invalid:\n' + JSON.stringify(data, null, 2))
    };
    callback({
      TmpSecretId: credentials.tmpSecretId,
      TmpSecretKey: credentials.tmpSecretKey,
      SecurityToken: credentials.sessionToken,
      StartTime: startTime,
      ExpiredTime: expiredTime,
    });
  }
});

function handleFileChange(e) {
  const file = e.target.files[0];
  if (!file) {
    return Promise.reject('no file');
  }
  const name = file.name;
  return new Promise((resolve, reject) => {
    cos.putObject(
      {
        Bucket: 'kimi-1251502833',
        Region: 'ap-beijing',
        Key: `cos-test/${name}`,
        Body: file,
        onProgress: function(progressData) {
          console.log('percentage', JSON.stringify(progressData));
        }
      },
      function(err, data) {
        if (err) {
          reject(err);
          console.error('upload fail', err);
        } else {
          resolve(data);
          console.info('upload success', data);
        }
      },
    );
  })
}

wasm().then(({ run_app }) => {
  run_app(handleFileChange);
});
