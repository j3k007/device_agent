from django.urls import path
from . import views

urlpatterns = [
    path('tokens/', views.list_tokens, name='list_tokens'),
    path('tokens/create/', views.create_token, name='create_token'),
]