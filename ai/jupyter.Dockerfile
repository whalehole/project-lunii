FROM quay.io/jupyter/base-notebook
#FROM quay.io/jupyter/pytorch-notebook:cuda12-python-3.11.8
#FROM quay.io/jupyter/tensorflow-notebook:cuda-latest
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt \
    && rm requirements.txt